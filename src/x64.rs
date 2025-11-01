use accui64r_macncheese::find_opcode;

use crate::{debug::get_reg_name, ram::RAM, reg::{CR0, CR4, DataTableReg, ModRM, REXBit, Reg, RegType, SegReg, XMMReg, get_size}};
use std::{collections::HashMap};
use crate::opcodes::std_ops::*;

pub fn get_mask(val: u64, start: u8, end: u8) -> u64 {
    let mut mask: u64 = (2 << start) - 1;
    if end != 0 {
        mask -= (2 << (end - 1)) - 1;
    }

    return (val & mask) >> end;
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum InfoType {
    REX,
    OPERAND,
    ADDRESS,
    ES, CS, SS, DS, FS, GS,
    NULL,
}

pub struct Info {
    pub val: u8,
    pub pos: u8,
}

pub struct CPU {
    running: bool,

    pub curr_inst: u8,
    pub info: HashMap<InfoType, Info>,

    pub regs: [Reg; 16],
    pub st_regs: [SegReg; 6],
    pub mm_regs: [u64; 8],
    pub xm_regs: [XMMReg; 16],
    pub cr_regs: [u64; 16],
    pub db_regs: [u32; 8],
    pub tr_regs: [u32; 8],

    pub ip: Reg,
    pub flags: u64,

    pub gdtr: DataTableReg,
    pub ldtr: u16,
    pub tr: u16,
    pub idtr: DataTableReg,

    // model-specific
    pub ia32_efer: u32,

    pub fs_base: u32,
    pub gs_base: u32,
    pub kernel_gs_base: u32,
}

pub enum OperandSize {
    BIT16,
    BIT32,
    BIT64,
}
pub enum AddressSize {
    BIT16,
    BIT32,
    BIT64,
}

impl CPU {
    pub fn new() -> Self {
        let default_st_reg: SegReg = SegReg { selector: 0, base: 0, limit: 0xFFFF, attr: 0 };

        let mut ret = Self {
            running: true,

            curr_inst: 0,
            info: HashMap::new(),

            regs: [Reg::new(); 16],
            st_regs: [default_st_reg.clone(); 6],
            mm_regs: [0u64; 8],
            xm_regs: [XMMReg { valn: [0u64; 8] }; 16],
            cr_regs: [50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            db_regs: [0, 0, 0, 0, 0, 0, 0xFFFF0FF0, 0x400],
            tr_regs: [0u32; 8],

            ip: Reg { val: 0xFFF0 },
            flags: 0,

            gdtr: DataTableReg { limit: 0, base: 0xFFFF },
            ldtr: 0,
            tr: 0,
            idtr: DataTableReg { limit: 0, base: 0xFFFF },
            ia32_efer: 0,

            fs_base: 0xC0000100,
            gs_base: 0xC0000101,
            kernel_gs_base: 0xC0000102,
        };

        ret.regs[3].val = 0x600;
        ret.st_regs[1].selector = 0xF000; // = SegReg { selector: 0xF000, base: 0xFFFF0000, limit: 0xFFFF, attr: 0 };
        ret.st_regs[1].base = 0xFFFF0000;

        ret
    }

    pub fn deter_op_size(&self) -> OperandSize {
        if self.info.contains_key(&InfoType::OPERAND) ^ !self.is_16bit_mode() {
            return OperandSize::BIT16;
        }

        if self.info.contains_key(&InfoType::REX) {
            let rex= self.info.get(&InfoType::REX).unwrap();
            if rex.val & REXBit::W as u8 != 0 {
                return OperandSize::BIT64;
            }
        }

        return OperandSize::BIT32;
    }

    pub fn deter_ad_size(&self) -> AddressSize {
        if (self.is_16bit_mode() && !self.info.contains_key(&InfoType::ADDRESS)) || (self.is_32bit_mode() && self.info.contains_key(&InfoType::ADDRESS)) {
            return AddressSize::BIT16;
        }
        if self.is_32bit_mode() || ((self.is_16bit_mode() || self.is_64bit_mode()) && self.info.contains_key(&InfoType::ADDRESS)) {
            return AddressSize::BIT32;
        }

        AddressSize::BIT64
    }

    pub fn cs(&self) -> SegReg { self.st_regs[1] }
    pub fn ds(&self) -> SegReg { self.st_regs[3] }

    pub fn get_seg_override_idx(&self) -> Option<u8> {
        let mut ret: Option<u8> = None;
        let mut seg: Option<&Info> = None;

        if self.info.contains_key(&InfoType::ES) {
            ret = Some(0);
            seg = self.info.get(&InfoType::ES);
        }
        if self.info.contains_key(&InfoType::CS) {
            let temp_info = self.info.get(&InfoType::CS).unwrap();
            if seg.is_none() || seg.unwrap().pos < temp_info.pos {
                ret = Some(1);
                seg = self.info.get(&InfoType::CS);
            }
        }
        if self.info.contains_key(&InfoType::SS) {
            let temp_info = self.info.get(&InfoType::SS).unwrap();
            if seg.is_none() || seg.unwrap().pos < temp_info.pos {
                ret = Some(2);
                seg = self.info.get(&InfoType::SS);
            }
        }
        if self.info.contains_key(&InfoType::DS) {
            let temp_info = self.info.get(&InfoType::DS).unwrap();
            if seg.is_none() || seg.unwrap().pos < temp_info.pos {
                ret = Some(3);
                seg = self.info.get(&InfoType::DS);
            }
        }
        if self.info.contains_key(&InfoType::FS) {
            let temp_info = self.info.get(&InfoType::FS).unwrap();
            if seg.is_none() || seg.unwrap().pos < temp_info.pos {
                ret = Some(4);
                seg = self.info.get(&InfoType::FS);
            }
        }
        if self.info.contains_key(&InfoType::GS) {
            let temp_info = self.info.get(&InfoType::GS).unwrap();
            if seg.is_none() || seg.unwrap().pos < temp_info.pos {
                ret = Some(5);
            }
        }

        ret
    }

    fn curr_virt_loc(&self) -> usize {
        ((self.cs().base as u64) + self.ip.val) as usize
    }

    pub fn read(&mut self, ram: &RAM) -> u8 {
        let ret = ram.read(self.curr_virt_loc());
        self.ip.val += 1;
        
        ret
    }

    pub fn get(&self, ram: &RAM, addr: u64) -> u8 {
        let idx: u8 = self.get_seg_override_idx().or(Some(3)).unwrap();
        let base: u32 = self.st_regs[idx as usize].base;

        ram.read(base as usize + addr as usize)
    }

    pub fn write(&self, ram: &mut RAM, addr: usize, val: u64, size: usize) {
        let mut base: u32 = self.st_regs[3].base; // DS
        if self.info.contains_key(&InfoType::CS) { base = self.st_regs[1].base }
        
        let addr: usize = base as usize + addr;
        for i in 0..size {
            ram.write(addr + i, (val >> (i * 8)) as u8);
        }
    }

    pub fn write_reg(&self, ram: &mut RAM, addr: u64, reg: u8, reg_type: RegType) {
        let val: u64 = self.regs[reg as usize].get(reg_type);
        let size: usize = get_size(reg_type);

        self.write(ram, addr as usize, val, size);
    }

    pub fn read_val8(&mut self, ram: &mut RAM) -> u8 {
        self.read(ram)
    }

    pub fn read_val16(&mut self, ram: &mut RAM) -> u16 {
        self.read(ram) as u16 + ((self.read(ram) as u16) << 8)
    }

    pub fn read_val32(&mut self, ram: &mut RAM) -> u32 {
        self.read(ram) as u32
          + ((self.read(ram) as u32) << 8)
          + ((self.read(ram) as u32) << 16)
          + ((self.read(ram) as u32) << 24)
    }

    pub fn read_val64(&mut self, ram: &mut RAM) -> u64 {
        self.read(ram) as u64
          + ((self.read(ram) as u64) << 8)
          + ((self.read(ram) as u64) << 16)
          + ((self.read(ram) as u64) << 24)
          + ((self.read(ram) as u64) << 32)
          + ((self.read(ram) as u64) << 40)
          + ((self.read(ram) as u64) << 48)
          + ((self.read(ram) as u64) << 56)
    }

    pub fn get_val8(&mut self, ram: &mut RAM, addr: u64) -> u8 {
        self.get(ram, addr)
    }

    pub fn get_val16(&mut self, ram: &mut RAM, addr: u64) -> u16 {
        self.get(ram, addr) as u16 + ((self.get(ram, addr + 1) as u16) << 8)
    }

    pub fn get_val32(&mut self, ram: &mut RAM, addr: u64) -> u32 {
        self.get(ram, addr) as u32
          + ((self.get(ram, addr + 1) as u32) << 8)
          + ((self.get(ram, addr + 2) as u32) << 16)
          + ((self.get(ram, addr + 3) as u32) << 24)
    }

    pub fn get_val64(&mut self, ram: &mut RAM, addr: u64) -> u64 {
        self.get(ram, addr) as u64
          + ((self.get(ram, addr + 1) as u64) << 8)
          + ((self.get(ram, addr + 2) as u64) << 16)
          + ((self.get(ram, addr + 3) as u64) << 24)
          + ((self.get(ram, addr + 4) as u64) << 32)
          + ((self.get(ram, addr + 5) as u64) << 40)
          + ((self.get(ram, addr + 6) as u64) << 48)
          + ((self.get(ram, addr + 7) as u64) << 56)
    }


    pub fn is_16bit_mode(&self) -> bool {
        !CR0::get_flag(self.cr_regs[0], CR0::PROT_MODE)
    }
    
    pub fn is_32bit_mode(&self) -> bool {
        CR0::get_flag(self.cr_regs[0], CR0::PROT_MODE) && !CR4::get_flag(self.cr_regs[4], CR4::PHYS_ADDR_EXT)
    }

    pub fn is_64bit_mode(&self) -> bool {
        CR0::get_flag(self.cr_regs[0], CR0::PROT_MODE)
            && CR0::get_flag(self.cr_regs[0], CR0::PAGING)
            && CR4::get_flag(self.cr_regs[4], CR4::PHYS_ADDR_EXT)
    }


    fn dbp_start_new_inst(&self) {
        println!("---------------------------");
        println!("EIP: {:0>8X}", self.curr_virt_loc());
    }

    pub fn run(&mut self, ram: &mut RAM) {
        self.dbp_start_new_inst();

        while self.running {
            if self.run_step(ram) { continue; }

            self.dbp_regs();
            self.dbp_start_new_inst();
        }
    }

    fn run_step(&mut self, ram: &mut RAM) -> bool {
        self.curr_inst = self.read(ram);

        let curr_inst = self.curr_inst;
        let self2 = self;

        find_opcode!(curr_inst, self2, ram)
    }

    fn dbp_regs(&self) {
        println!();
        for i in [0, 4, 8, 0xC] {
            print!("{:>3}: {:016X}  ", get_reg_name(i + 0, RegType::R64), self.regs[i as usize + 0].get(RegType::R64));
            print!("{:>3}: {:016X}  ", get_reg_name(i + 1, RegType::R64), self.regs[i as usize + 1].get(RegType::R64));
            print!("{:>3}: {:016X}  ", get_reg_name(i + 2, RegType::R64), self.regs[i as usize + 2].get(RegType::R64));
            println!("{:>3}: {:016X}", get_reg_name(i + 3, RegType::R64), self.regs[i as usize + 3].get(RegType::R64));
        }
        println!();

        println!("IDTR: size: {}, addr: {}", self.idtr.base, self.idtr.limit);
    }

    pub fn get_reg_ptr(&self, idx: u8, reg_type: RegType) -> u64 {
        let mut val: u64 = 0;

        let segidx = self.get_seg_override_idx();
        if !segidx.is_none() {
            val += self.st_regs[segidx.unwrap() as usize].base as u64;
        }

        val += self.regs[idx as usize].get(reg_type);

        val
    }

    pub fn get_modrm_ptr(&mut self, ram: &mut RAM, modrm: &ModRM, disp: &mut u32) -> u64 {
        let mut val: u64 = 0;

        if !modrm.should_use_sib() {
            if !modrm.rm_type.is_none() {
                let rm_type: RegType = modrm.rm_type.unwrap();

                val += self.regs[modrm.rm as usize].get(rm_type);
            }
        } else {
            if !modrm.sib.idx_type.is_none() {
                let idx_type: RegType = modrm.sib.idx_type.unwrap();
                
                val += self.regs[modrm.sib.idx as usize].get(idx_type);
                val *= modrm.sib.mul as u64;
            }
            if !modrm.sib.base_type.is_none() {
                let base_type: RegType = modrm.sib.base_type.unwrap();
                
                val += self.regs[modrm.sib.base as usize].get(base_type);
            }
        }

        match modrm.disp {
            1 => {
                *disp = self.read_val8(ram) as u32;
                val += *disp as u64;
            },
            2 => {
                *disp = self.read_val16(ram) as u32;
                val += *disp as u64;
            },
            4 => {
                *disp = self.read_val32(ram) as u32;
                val += *disp as u64;
            },
            _ => {}
        }

        val
    }

    pub fn get_modrm(&mut self, ram: &mut RAM, reg_type: RegType) -> ModRM {
        let val: u8 = self.read(ram);

        if self.is_16bit_mode() || (self.is_32bit_mode() && self.info.contains_key(&InfoType::ADDRESS)) {
            return self.get_modrm16(val, reg_type);
        } else {
            return self.get_modrm32(val, ram, reg_type);
        }
    }

    pub fn get_ptr16(&self, modrm: &mut ModRM) {
        match modrm._rm {
            0 => { modrm.sib.idx = 3; modrm.sib.base = 4; }
            1 => { modrm.sib.idx = 3; modrm.sib.base = 5; }
            2 => { modrm.sib.idx = 7; modrm.sib.base = 4; }
            3 => { modrm.sib.idx = 7; modrm.sib.base = 5; }
            4 => { modrm.rm = 4; }
            5 => { modrm.rm = 5; }
            6 => { modrm.rm = 7; }
            7 => { modrm.rm = 3; }
            _ => {}
        }
    }

    fn get_modrm16(&mut self, val: u8, reg_type: RegType) -> ModRM {
        let mut modrm = ModRM::new(val.into());
        
        modrm.disp = modrm._mod % 3;

        if modrm._mod == 3 {
            self.deter_modrm16(&mut modrm, reg_type);
            modrm.rm = modrm._rm;
            modrm.rm_type = modrm.reg_type;

            return modrm;
        }

        self.get_ptr16(&mut modrm);

        if modrm._mod == 0 && modrm._rm == 6 {
            modrm.rm_type = None;
            modrm.disp = 2;
        } else if self.info.contains_key(&InfoType::ADDRESS) {
            modrm.rm_type = Some(RegType::R32);
            modrm.sib.idx_type = Some(RegType::R32);
            modrm.sib.base_type = Some(RegType::R32);
        } else {
            modrm.rm_type = Some(RegType::R16);
            modrm.sib.idx_type = Some(RegType::R16);
            modrm.sib.base_type = Some(RegType::R16);
        }

        self.deter_modrm16(&mut modrm, reg_type);
        modrm
    }

    fn get_modrm32(&mut self, val: u8, ram: &RAM, reg_type: RegType) -> ModRM {
        let mut modrm = ModRM::new(val.into());

        match modrm._mod {
            0 => modrm.disp = 0,
            1 => modrm.disp = 1,
            2 => modrm.disp = 4,
            3 => { self.deter_modrm32_mod(&mut modrm, reg_type); return modrm; },
            _ => {},
        }

        if modrm._rm == 4 {
            self.deter_modrm32_sib(ram, &mut modrm, reg_type);
            return modrm;
        }

        if modrm._rm == 5 && modrm._mod == 0 {
            modrm.rm = 17;
            modrm.disp = 4;
        }

        self.deter_modrm32_mod(&mut modrm, reg_type);
        modrm
    }

    fn set_modrm_idx(modrm: &mut ModRM, rmidx: u8, regidx: u8) {
        modrm.rm  = rmidx;
        modrm.reg = regidx;
    }

    fn deter_modrm16(&self, modrm: &mut ModRM, reg_type: RegType) {
        match reg_type {
            RegType::R8 | RegType::R8H => {
                modrm.reg_type = Some(RegType::R8);
                if modrm._reg >= 4 && modrm._reg < 8 {
                    modrm._reg -= 4;
                    modrm.reg_type = Some(RegType::R8H);
                }
                
                modrm.reg = modrm._reg;
            }

            RegType::R16 | RegType::R32 => {
                modrm.reg_type = Some(RegType::R16);
                if self.info.contains_key(&InfoType::OPERAND) {
                    modrm.reg_type = Some(RegType::R32);
                }
                
                modrm.reg = modrm._reg;
            }
            
            _ => {},
        }
    }

    fn deter_modrm32_mod(&self, modrm: &mut ModRM, reg_type: RegType) {
        let rex = self.info.get(&InfoType::REX).or(Some(&Info { val: 0, pos: 0 })).unwrap();
        let rmidx:  u8 = modrm._rm  | (((rex.val & REXBit::B as u8 != 0) as u8) << 3);
        let regidx: u8 = modrm._reg | (((rex.val & REXBit::R as u8 != 0) as u8) << 3);

        modrm.reg_type = Some(reg_type);
        match reg_type {
            RegType::R8 | RegType::R8H => {
                modrm.rm_type  = Some(RegType::R8);
                modrm.reg_type = Some(RegType::R8);
                if modrm._reg >= 4 && modrm._reg < 8 && !self.info.contains_key(&InfoType::REX) {
                    modrm._reg -= 4;
                    modrm.reg_type = Some(RegType::R8H);
                }
                if modrm._rm >= 4 && modrm._rm < 8 {
                    modrm._rm -= 4;
                    modrm.rm_type = Some(RegType::R8H);
                }
                CPU::set_modrm_idx(modrm, rmidx, regidx);
            }

            RegType::R16 | RegType::R32 | RegType::R64 => {
                modrm.reg_type = if self.info.contains_key(&InfoType::OPERAND) { Some(RegType::R16)
                                 } else if (rex.val & REXBit::W as u8) != 0 { Some(RegType::R64)
                                 } else { Some(RegType::R32) };
                
                CPU::set_modrm_idx(modrm, rmidx, regidx);
            }

            RegType::ST  => { CPU::set_modrm_idx(modrm, rmidx & 7, regidx & 7); }
            RegType::MM  => { CPU::set_modrm_idx(modrm, rmidx & 7, regidx & 7); }
            RegType::XMM => { CPU::set_modrm_idx(modrm, rmidx, regidx); }
        }
    }

    fn deter_modrm32_sib(&mut self, ram: &RAM, modrm: &mut ModRM, reg_type: RegType) {
        let sib: u8 = self.read(ram);

        modrm.sib._ss   = get_mask(sib as u64, 7, 6) as u8;
        modrm.sib._base = get_mask(sib as u64, 5, 3) as u8;
        modrm.sib._idx  = get_mask(sib as u64, 2, 0) as u8;

        let rex = self.info.get(&InfoType::REX).or(Some(&Info { val: 0, pos: 0 })).unwrap();
        let baseidx: u8 = modrm.sib._base | (((rex.val & REXBit::X as u8 != 0) as u8) << 3);
        let idxidx: u8  = modrm.sib._idx  | (((rex.val & REXBit::B as u8 != 0) as u8) << 3);
        let regidx: u8  = modrm._reg      | (((rex.val & REXBit::R as u8 != 0) as u8) << 3);

        modrm.sib.idx = idxidx;
        modrm.sib.base = baseidx;
        modrm.sib.mul = if modrm.sib._ss != 0 { modrm.sib._ss * 2 } else { 1 };

        if modrm.sib._base == 5 && modrm._mod == 0 {
            modrm.disp = 4;
            modrm.sib.base_type = None;
        }

        modrm.sib.idx_type = if self.is_32bit_mode() || self.info.contains_key(&InfoType::ADDRESS) { Some(RegType::R32) } else { Some(RegType::R64) };
        modrm.sib.base_type = modrm.sib.idx_type;

        modrm.reg_type = Some(reg_type);
        match reg_type {
            RegType::R8 | RegType::R8H => {
                modrm.reg_type = Some(RegType::R8);
                if modrm._reg >= 4 && modrm._reg < 8 && !self.info.contains_key(&InfoType::REX) {
                    modrm._reg -= 4;
                    modrm.reg_type = Some(RegType::R8H);
                }
                
                modrm.reg = regidx;
            }

            RegType::R16 | RegType::R32 | RegType::R64 => {
                modrm.reg_type = if self.info.contains_key(&InfoType::OPERAND) { Some(RegType::R16)
                                 } else if (rex.val & REXBit::W as u8) != 0 { Some(RegType::R64)
                                 } else { Some(RegType::R32) };
                
                modrm.reg = regidx;
            }

            RegType::ST => { modrm.reg = modrm._reg }
            RegType::MM => { modrm.reg = modrm._reg }
            RegType::XMM => { modrm.reg = regidx }
        }
    }
}