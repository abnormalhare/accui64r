use crate::{alu::get_bit_mask, ram, x64::{self, get_mask}};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RegType {
    R8,
    R8H,
    R16,
    R32,
    R64,

    ST,
    MM,
    XMM,
}

impl RegType {
    pub fn to_xmm_type(&self) -> XMMType {
        match *self {
            RegType::R32 => XMMType::N32,
            RegType::R64 => XMMType::N64,
            _ => XMMType::N128,
        }
    }
}

pub enum XMMType {
    N32,
    F32,
    N64,
    F64,
    N128,
    F128,
    N256,
    N512,
}

impl XMMType {
    pub fn to_reg_type(&self) -> RegType {
        match *self {
            XMMType::N32 => RegType::R32,
            XMMType::N64 => RegType::R64,
            _ => RegType::XMM,
        }
    }
}

#[derive(Clone, Copy)]
pub enum REXBit {
    B = 1,
    X = 2,
    R = 4,
    W = 8,
}

#[derive(Clone, Copy)]
pub enum ExceptionType {
    DE, DB, NMI, BP, OF, BR, UD, NM, DF, CSO,
    TS, NP, SS, GP, PF, _0, MF, AC, MC, XM,
    VE, CP, _1, HV, VC, SX, _3,
}

#[derive(Clone, Copy)]
pub struct Reg {
    pub val: u64
}

impl Reg {
    pub fn new() -> Self {
        Self { val: 0 }
    }
    
    pub fn new_ptr(ram: ram::RAM, ptr: usize) -> Self {
        Self { val: ram.read(ptr) as u64 }
    }

    pub fn get(&self, reg_type: RegType) -> u64 {
        match reg_type {
            RegType::R8H => return x64::get_mask(self.val, 15, 8).into(),
            _ => {
                let bits = get_size(reg_type) * 8;
                let mask: u64 = get_bit_mask(reg_type, bits);

                return self.val & mask;
            },
        }
    }

    pub fn set(&mut self, reg_type: RegType, val: u64) {
        match reg_type {
            RegType::R8H => {
                self.val &= !0xFF00;
                self.val |= (val & 0xFF) << 8;
            },
            _ => {
                let bits = get_size(reg_type) * 8;
                let mask: u64 = get_bit_mask(reg_type, bits);

                self.val &= !mask;
                self.val |= val & mask;
            },
        }
    }
}

pub fn get_size(reg_type: RegType) -> usize {
    match reg_type {
        RegType::R8 | RegType::R8H => 1,
        RegType::R16 => 2,
        RegType::R32 => 4,
        RegType::R64 => 8,
        _ => 0,
    }
}

#[derive(Clone, Copy)]
pub union XMMReg {
    pub valn: [u32; 16],
    pub valf: [f32; 16],
}

impl XMMReg {
    pub fn setn(&mut self, xmm_type: XMMType, val: u128) {
        unsafe {

        match xmm_type {
            XMMType::N32 => {
                self.valn[0] = val as u32;
                self.valn[1] = 0u32;
            },
            XMMType::N64 => {
                self.valn[0] = val as u32;
                self.valn[1] = (val >> 32) as u32;
            },
            XMMType::N128 => {
                self.valn[0] = val as u32;
                self.valn[1] = (val >> 32) as u32;
                self.valn[2] = (val >> 64) as u32;
                self.valn[3] = (val >> 96) as u32;
            },
            _ => {},
        }

        } // unsafe
    }

    pub fn setnl(&mut self, xmm_type: XMMType, val: [u32; 16]) {
        unsafe {

        match xmm_type {
            XMMType::N32 => self.valn[0] = val[0],
            XMMType::N64 => {
                self.valn[0] = val[0]; self.valn[1] = val[1];
            },
            XMMType::N128 => {
                self.valn[0] = val[0]; self.valn[1] = val[1];
                self.valn[2] = val[2]; self.valn[3] = val[3];
                self.valn[4] = val[4]; self.valn[5] = val[5];
                self.valn[6] = val[6]; self.valn[7] = val[7];
            },
            XMMType::N256 => {
                self.valn[0]  = val[0];  self.valn[1]  = val[1];
                self.valn[2]  = val[2];  self.valn[3]  = val[3];
                self.valn[4]  = val[4];  self.valn[5]  = val[5];
                self.valn[6]  = val[6];  self.valn[7]  = val[7];
                self.valn[8]  = val[8];  self.valn[9]  = val[9];
                self.valn[10] = val[10]; self.valn[11] = val[11];
                self.valn[12] = val[12]; self.valn[13] = val[13];
                self.valn[14] = val[14]; self.valn[15] = val[15];
            },
            XMMType::N512 => {
                self.valn = val;
            },
            _ => {},
        }

        } // unsafe
    }

    pub fn setf(&mut self, xmm_type: XMMType, val: [f32; 16]) {
        unsafe {

        match xmm_type {
            XMMType::N32 => self.valf[0] = val[0],
            XMMType::N64 => {
                self.valf[0] = val[0]; self.valf[1] = val[1];
            },
            XMMType::N128 => {
                self.valf[0] = val[0]; self.valf[1] = val[1];
                self.valf[2] = val[2]; self.valf[3] = val[3];
                self.valf[4] = val[4]; self.valf[5] = val[5];
                self.valf[6] = val[6]; self.valf[7] = val[7];
            },
            XMMType::N256 => {
                self.valf[0]  = val[0];  self.valf[1]  = val[1];
                self.valf[2]  = val[2];  self.valf[3]  = val[3];
                self.valf[4]  = val[4];  self.valf[5]  = val[5];
                self.valf[6]  = val[6];  self.valf[7]  = val[7];
                self.valf[8]  = val[8];  self.valf[9]  = val[9];
                self.valf[10] = val[10]; self.valf[11] = val[11];
                self.valf[12] = val[12]; self.valf[13] = val[13];
                self.valf[14] = val[14]; self.valf[15] = val[15];
            },
            XMMType::N512 => {
                self.valf = val;
            },
            _ => {},
        }

        } // unsafe
    }
}

#[derive(Clone, Copy)]
pub struct SegRegType {
    pub accessed: bool,
    pub write: bool,
    pub exp_down: bool,
    pub data_exec: bool,
}

#[derive(Clone, Copy)]
pub struct SegReg {
    pub selector: u16,  // actual number stored
    pub base: u32,      // true address
    pub limit: u32,     // max ip index + base
    pub stype: SegRegType, // info
    pub s: bool,           // if register is data (0) or code (1)
    pub dpl: u8,           // descriptor privilege level
    pub present: bool,     // 
    pub avl: bool,
    pub l: bool,
    pub db: bool,
    pub gran: bool,

    pub usable: bool,
}

impl SegReg {
    pub fn set(&mut self, val: u16) {
        self.selector = val;
    }
}

#[derive(Clone, Copy)]
pub struct DataTableReg {
    pub limit: u16,
    pub base: u64,
}

#[derive(Clone, Copy)]
pub struct TableReg {
    pub limit_lo: u16,
    pub base_lo: u16,
    pub base_mid: u8,
    pub flags: u8,
    pub limit_hi: u8,
    pub base_hi: u8,
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Flag {
    CARRY_FLAG  = 1 << 0,
    PARITY_FLAG = 1 << 2,
    AUX_FLAG    = 1 << 4,
    ZERO_FLAG   = 1 << 6,
    SIGN_FLAG   = 1 << 7,
    TRAP_FLAG   = 1 << 8,
    INT_FLAG    = 1 << 9,
    DIR_FLAG    = 1 << 10,
    OVER_FLAG   = 1 << 11,
    IOPL_FLAG   = 1 << 12,
    IOPL_FLAG2  = 1 << 13,
    NESTED_TASK = 1 << 14,
    RES_FLAG    = 1 << 16,
    VM_FLAG     = 1 << 17,
    ALIGN_CHECK = 1 << 18,
    VINT_FLAG   = 1 << 19,
    VINT_PEND   = 1 << 20,
    ID_FLAG     = 1 << 21,
}

impl Flag {
    pub fn get_flag(val: u64, flag: Flag) -> bool {
        (val & (flag as u64)) != 0
    }

    pub fn get_iopl_flag(val: u64) -> u8 {
        ((val & (Flag::IOPL_FLAG as u64)) != 0) as u8 + ((((val & (Flag::IOPL_FLAG as u64)) != 0) as u8) << 1)
    }

    pub fn set_flag(val: &mut u64, flag: Flag, new: bool) {
        *val &= !(flag as u64);
        *val |= (flag as u64) * (new as u64);
    }
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum CR0 {
    PROT_MODE      = 1 << 0,
    MON_COPROC     = 1 << 1,
    EMULATION      = 1 << 2,
    TASK_SWITCH    = 1 << 3,
    EXT_TYPE       = 1 << 4,
    NUM_ERROR      = 1 << 5,
    WRITE_PROT     = 1 << 16,
    ALIGN_MASK     = 1 << 18,
    NOT_WRITE_THRU = 1 << 29,
    CACHE_DISABLE  = 1 << 30,
    PAGING         = 1 << 31,
}

impl CR0 {
    pub fn get_flag(val: u64, flag: CR0) -> bool {
        (val & (flag as u64)) != 0
    }

    pub fn set_flag(val: &mut u64, flag: CR0, new: bool) {
        *val &= !(flag as u64);
        *val |= (flag as u64) * (new as u64);
    }
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum CR4 {
    V8086_EXT     = 1 << 0,
    PROTMODE_VINT = 1 << 1,
    TIME_STAMP    = 1 << 2,
    DEBUG_EXT     = 1 << 3,
    PAGE_SIZE_EXT = 1 << 4,
    PHYS_ADDR_EXT = 1 << 5,
    MACH_CHK_EXT  = 1 << 6,
    PAGE_GLOB     = 1 << 7,
    PERF_COUNT    = 1 << 8,
    OSFXSR        = 1 << 9,
    OSXMMEXCPT    = 1 << 10,
    UMIP          = 1 << 11,
    FIVE_LVL_PAGE = 1 << 12,
    VM_EXT        = 1 << 13,
    SAFE_MODE_EXT = 1 << 14,
    FSGSBASE      = 1 << 16,
    PCID_ENABLE   = 1 << 17,
    OSXSAVE       = 1 << 18,
    SV_MODE_EXEC  = 1 << 20,
    SV_MODE_ACC   = 1 << 21,
    PROT_KEY_USR  = 1 << 22,
    CONTROL_ENF   = 1 << 23,
    PROT_KEY_SV   = 1 << 24,
}

impl CR4 {
    pub fn get_flag(val: u64, flag: CR4) -> bool {
        (val & (flag as u64)) != 0
    }

    pub fn set_flag(val: &mut u64, flag: CR4, new: bool) {
        *val &= !(flag as u64);
        *val |= (flag as u64) * (new as u64);
    }
}

#[derive(Clone, Copy)]
pub struct SIB {
    pub _ss: u8,
    pub _base: u8,
    pub _idx: u8,

    pub idx: u8,
    pub idx_type: Option<RegType>,

    pub base: u8,
    pub base_type: Option<RegType>,

    pub mul: u8,
}

#[derive(Clone, Copy)]
pub struct ModRM {
    pub _mod: u8,
    pub _reg: u8,
    pub _rm: u8,

    pub rm: u8,
    pub rm_type: Option<RegType>,

    pub reg: u8,
    pub reg_type: Option<RegType>,

    pub disp: u8,
    pub sib: SIB,
}

impl ModRM {
    pub fn new(val: u64) -> Self {
        Self {
            _mod: get_mask(val.into(), 7, 6) as u8,
            _reg: get_mask(val.into(), 5, 3) as u8,
            _rm: get_mask(val.into(), 2, 0) as u8,
            
            rm: 0, rm_type: None,
            reg: 0, reg_type: None,
            disp: 0,
            sib: SIB {
                _ss: 0, _base: 0, _idx: 0,

                idx: 0, idx_type: None,
                base: 0, base_type: None,
                mul: 1,
            },
        }
    }

    pub fn get_val(&self) -> u8 {
        (self._mod << 6) + (self._reg << 3) + self._rm
    }

    pub fn rm_is_reg(&self) -> bool {
        self._mod == 3
    }

    pub fn should_use_sib(&self) -> bool {
        self.rm_type.is_none()
    }

    pub fn get_ptr_type(&self) -> Option<RegType> {
        if self.should_use_sib() {
            if !self.sib.idx_type.is_none() {
                return self.sib.idx_type;
            } else if !self.sib.base_type.is_none() {
                return self.sib.base_type;
            } else {
                return None;
            }
        } else {
            return self.rm_type;
        }
    }

    pub fn set_ptr_type(&mut self, reg_type: Option<RegType>) {
        if self.should_use_sib() {
            if !self.sib.idx_type.is_none() {
                self.sib.idx_type = reg_type;
            } else if !self.sib.base_type.is_none() {
                self.sib.base_type = reg_type;
            } else {
                self.sib.idx_type = reg_type;
            }
        } else {
            self.rm_type = reg_type;
        }
    }
}