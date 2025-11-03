use crate::{
    alu,
    debug::{OpOrder, dbp, get_reg_name},
    opcodes::{x83_ops::OP_83_TBL, xc1_ops::OP_C1_TBL, xff_ops::OP_FF_TBL},
    ram::RAM,
    reg::{CR4, Flag, ModRM, RegType, get_size},
    x64::{CPU, Info, InfoType, OperandSize}
};
use accui64r_macncheese::find_opcode_0f;

pub type OpFunc    = fn(&mut CPU, &mut RAM, &ModRM);
pub type OpFuncRet = fn(&mut CPU, &mut RAM, &ModRM) -> bool;

// pub fn rm_handler_example(cpu: &mut CPU, ram: &mut RAM) -> bool {
//     let mut modrm: ModRM = cpu.get_modrm(ram, RegType::R16);
//     let mut disp: u32 = 0;

//     if modrm.rm_is_reg() {

//     } else {
//        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);

//     }

//     false
// }

pub fn op_00(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R8);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.reg as usize].get(reg_type);
        let dst = cpu.regs[modrm.rm  as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.rm as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.write(ram, addr as usize, res, get_size(reg_type));
    }

    dbp(cpu, "ADD", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_01(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.reg as usize].get(reg_type);
        let dst = cpu.regs[modrm.rm  as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.rm as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.write(ram, addr as usize, res, get_size(reg_type));
    }

    dbp(cpu, "ADD", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_02(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R8);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.rm  as usize].get(reg_type);
        let dst = cpu.regs[modrm.reg as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.reg as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.reg as usize].set(reg_type, res);
    }

    dbp(cpu, "ADD", &modrm, disp, 0, OpOrder::R_RM);

    false
}

pub fn op_03(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.rm  as usize].get(reg_type);
        let dst = cpu.regs[modrm.reg as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.reg as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::add(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.reg as usize].set(reg_type, res);
    }

    dbp(cpu, "ADD", &modrm, disp, 0, OpOrder::R_RM);

    false
}

pub fn op_04(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let dst: u64 = cpu.regs[0].get(RegType::R8);
    let val: u64 = cpu.read_val8(ram).into();

    let res: u64 = alu::add(cpu, RegType::R8, dst, val);
    cpu.regs[0].set(RegType::R8, res);

    println!("ADD AL, {:0>2X}", val);

    false
}

pub fn op_05(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let reg_type = cpu.deter_op_size().to_reg_type();
    
    let dst: u64 = cpu.regs[0].get(reg_type);
    let val: u64 = if reg_type == RegType::R16 { cpu.read_val16(ram) as u64 } else { cpu.read_val32(ram) as u64 };

    let res = alu::add(cpu, reg_type, dst, val);
    cpu.regs[0].set(reg_type, res);

    match reg_type {
        RegType::R16   => println!("ADD {}, 0x{:0>4X}", get_reg_name(0, reg_type), val),
        RegType::R32 |
          RegType::R64 => println!("ADD {}, 0x{:0>8X}", get_reg_name(0, reg_type), val),
        _ => {},
    }

    false
}

pub fn op_06(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 06");
}
pub fn op_07(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 07");
}
pub fn op_08(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 08");
}
pub fn op_09(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 09");
}
pub fn op_0a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0A");
}
pub fn op_0b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0B");
}
pub fn op_0c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0C");
}

pub fn op_0d(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let reg_type = cpu.deter_op_size().to_reg_type();
    
    let dst: u64 = cpu.regs[0].get(reg_type);
    let val: u64 = if reg_type == RegType::R16 { cpu.read_val16(ram) as u64 } else { cpu.read_val32(ram) as u64 };

    let res = alu::or(cpu, reg_type, dst, val);
    cpu.regs[0].set(reg_type, res);

    match reg_type {
        RegType::R16   => println!("OR {}, 0x{:0>4X}", get_reg_name(0, reg_type), val),
        RegType::R32 |
          RegType::R64 => println!("OR {}, 0x{:0>8X}", get_reg_name(0, reg_type), val),
        _ => {},
    }

    false
}

pub fn op_0e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0E");
}

use crate::opcodes::x0f_ops::*;
pub fn op_0f(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let subop: u8 = cpu.read(ram);

    find_opcode_0f!(subop, cpu, ram)
}

pub fn op_10(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 10");
}
pub fn op_11(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 11");
}
pub fn op_12(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 12");
}
pub fn op_13(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 13");
}
pub fn op_14(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 14");
}
pub fn op_15(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 15");
}
pub fn op_16(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 16");
}
pub fn op_17(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 17");
}
pub fn op_18(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 18");
}
pub fn op_19(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 19");
}
pub fn op_1a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1A");
}
pub fn op_1b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1B");
}
pub fn op_1c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1C");
}
pub fn op_1d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1D");
}
pub fn op_1e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1E");
}
pub fn op_1f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 1F");
}
pub fn op_20(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 20");
}
pub fn op_21(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 21");
}
pub fn op_22(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 22");
}
pub fn op_23(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 23");
}
pub fn op_24(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 24");
}

pub fn op_25(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let reg_type = cpu.deter_op_size().to_reg_type();
    
    let dst: u64 = cpu.regs[0].get(reg_type);
    let val: u64 = if reg_type == RegType::R16 { cpu.read_val16(ram) as u64 } else { cpu.read_val32(ram) as u64 };

    let res = alu::and(cpu, reg_type, dst, val);
    cpu.regs[0].set(reg_type, res);

    match reg_type {
        RegType::R16   => println!("AND {}, 0x{:0>4X}", get_reg_name(0, reg_type), val),
        RegType::R32 |
          RegType::R64 => println!("AND {}, 0x{:0>8X}", get_reg_name(0, reg_type), val),
        _ => {},
    }

    false
}

pub fn op_26(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 26");
}
pub fn op_27(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 27");
}
pub fn op_28(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 28");
}

pub fn op_29(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R16);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.reg as usize].get(reg_type);
        let dst = cpu.regs[modrm.rm  as usize].get(reg_type);
        let res = alu::sub(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.rm as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::sub(cpu, reg_type, dst, src);
        
        cpu.write(ram, addr as usize, res, get_size(reg_type));
    }

    dbp(cpu, "SUB", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_2a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 2A");
}
pub fn op_2b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 2B");
}
pub fn op_2c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 2C");
}
pub fn op_2d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 2D");
}

pub fn op_2e(cpu: &mut CPU, _ram: &mut RAM) -> bool {
    let pos: u8 = cpu.info.len() as u8;

    cpu.info.insert(
        InfoType::CS, Info { val: 0x2E, pos: pos }
    );

    true
}

pub fn op_2f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 2F");
}
pub fn op_30(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 30");
}

pub fn op_31(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R16);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        let src = cpu.regs[modrm.reg as usize].get(reg_type);
        let dst = cpu.regs[modrm.rm  as usize].get(reg_type);
        let res = alu::xor(cpu, reg_type, dst, src);
        
        cpu.regs[modrm.rm as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        
        let src = cpu.get(ram, addr) as u64;
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::xor(cpu, reg_type, dst, src);
        
        cpu.write(ram, addr as usize, res, get_size(reg_type));
    }

    dbp(cpu, "XOR", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_32(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 32");
}
pub fn op_33(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 33");
}
pub fn op_34(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 34");
}
pub fn op_35(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 35");
}
pub fn op_36(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 36");
}
pub fn op_37(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 37");
}
pub fn op_38(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 38");
}
pub fn op_39(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 39");
}
pub fn op_3a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3A");
}
pub fn op_3b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3B");
}
pub fn op_3c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3C");
}
pub fn op_3d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3D");
}
pub fn op_3e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3E");
}
pub fn op_3f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 3F");
}
pub fn op_40(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 40");
}
pub fn op_41(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 41");
}
pub fn op_42(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 42");
}
pub fn op_43(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 43");
}
pub fn op_44(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 44");
}
pub fn op_45(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 45");
}
pub fn op_46(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 46");
}
pub fn op_47(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 47");
}
pub fn op_48(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 48");
}
pub fn op_49(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 49");
}
pub fn op_4a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4A");
}
pub fn op_4b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4B");
}
pub fn op_4c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4C");
}
pub fn op_4d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4D");
}
pub fn op_4e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4E");
}
pub fn op_4f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 4F");
}
pub fn op_50(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 50");
}
pub fn op_51(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 51");
}
pub fn op_52(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 52");
}
pub fn op_53(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 53");
}
pub fn op_54(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 54");
}
pub fn op_55(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 55");
}
pub fn op_56(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 56");
}
pub fn op_57(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 57");
}
pub fn op_58(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 58");
}
pub fn op_59(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 59");
}
pub fn op_5a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5A");
}
pub fn op_5b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5B");
}
pub fn op_5c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5C");
}
pub fn op_5d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5D");
}
pub fn op_5e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5E");
}
pub fn op_5f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 5F");
}
pub fn op_60(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 60");
}
pub fn op_61(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 61");
}
pub fn op_62(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 62");
}
pub fn op_63(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 63");
}
pub fn op_64(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 64");
}
pub fn op_65(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 65");
}

pub fn op_66(cpu: &mut CPU, _ram: &mut RAM) -> bool {
    let pos: u8 = cpu.info.len() as u8;

    cpu.info.insert(
        InfoType::OPERAND, Info { val: 0x66, pos: pos }
    );

    true
}

pub fn op_67(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 67");
}
pub fn op_68(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 68");
}
pub fn op_69(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 69");
}
pub fn op_6a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6A");
}
pub fn op_6b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6B");
}
pub fn op_6c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6C");
}
pub fn op_6d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6D");
}
pub fn op_6e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6E");
}
pub fn op_6f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 6F");
}
pub fn op_70(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 70");
}
pub fn op_71(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 71");
}
pub fn op_72(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 72");
}
pub fn op_73(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 73");
}
pub fn op_74(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 74");
}
pub fn op_75(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let rel = cpu.read_val8(ram);
    let do_jump = Flag::get_flag(cpu.flags, Flag::ZERO_FLAG) == false;

    if do_jump {
        let reg_type = cpu.get_reg_type_from_mode();
        let val = cpu.ip.get(reg_type) + rel as u64;
        cpu.ip.set(reg_type, val);
    }

    println!("JNZ {:0>2X}", rel);

    false
}
pub fn op_76(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 76");
}
pub fn op_77(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 77");
}
pub fn op_78(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 78");
}
pub fn op_79(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 79");
}
pub fn op_7a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7A");
}
pub fn op_7b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7B");
}
pub fn op_7c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7C");
}
pub fn op_7d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7D");
}
pub fn op_7e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7E");
}
pub fn op_7f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 7F");
}
pub fn op_80(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 80");
}
pub fn op_81(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 81");
}
pub fn op_82(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 82");
}
pub fn op_83(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);

    OP_83_TBL[modrm._reg as usize](cpu, ram, &modrm);

    false
}
pub fn op_84(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 84");
}
pub fn op_85(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 85");
}
pub fn op_86(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 86");
}
pub fn op_87(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 87");
}
pub fn op_88(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 88");
}

pub fn op_89(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        cpu.regs[modrm.rm as usize].set(reg_type, cpu.regs[modrm.reg as usize].get(reg_type));
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        cpu.write_reg(ram, addr, modrm.reg, reg_type);
    }

    dbp(cpu, "MOV", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_8a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 8A");
}
pub fn op_8b(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();

    if modrm.rm_is_reg() {
        cpu.regs[modrm.reg as usize].set(reg_type, cpu.regs[modrm.rm as usize].get(reg_type));
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        let val: u64 = match reg_type {
            RegType::R16 => cpu.get_val16(ram, addr) as u64,
            RegType::R32 => cpu.get_val32(ram, addr) as u64,
            RegType::R64 => cpu.get_val64(ram, addr),
            _ => 0,
        };

        cpu.regs[modrm.reg as usize].set(reg_type, val);
    }

    dbp(cpu, "MOV", &modrm, disp, 0, OpOrder::R_RM);

    false
}

pub fn op_8c(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let mut modrm: ModRM = cpu.get_modrm(ram, RegType::R32);
    let mut disp: u32 = 0;
    
    let val: u16 = cpu.sg_regs[modrm.reg as usize].selector;

    if cpu.effective_privilege_level() > cpu.sg_regs[modrm.reg as usize].dpl {
        todo!("Added Protection Faults")
    }

    if modrm.rm_is_reg() {
        let reg_type: RegType = modrm.reg_type.unwrap();

        cpu.regs[modrm.rm as usize].set(reg_type, val.into());
    } else {
        let reg_type: RegType = RegType::R16;
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);

        cpu.write(ram, addr as usize, val.into(), get_size(reg_type));
    }

    modrm.reg_type = Some(RegType::ST);

    dbp(cpu, "MOV", &modrm, disp, 0, OpOrder::RM_R);

    false
}

pub fn op_8d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 8D");
}
pub fn op_8e(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let mut modrm: ModRM = cpu.get_modrm(ram, RegType::R16);
    let mut disp: u32 = 0;
    
    if cpu.effective_privilege_level() > cpu.sg_regs[modrm.reg as usize].dpl {
        todo!("Added Protection Faults")
    }

    let reg_type = RegType::R16;
    if modrm.rm_is_reg() {
        let val: u16 = cpu.regs[modrm.rm as usize].get(reg_type) as u16;
        
        cpu.sg_regs[modrm.reg as usize].set(val);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        let val: u16 = cpu.get_val16(ram, addr);
        
        cpu.sg_regs[modrm.reg as usize].set(val);
    }

    modrm.reg_type = Some(RegType::ST);
    modrm.rm_type = Some(RegType::R16);

    dbp(cpu, "MOV", &modrm, disp, 0, OpOrder::R_RM);

    false
}
pub fn op_8f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 8F");
}
pub fn op_90(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 90");
}
pub fn op_91(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 91");
}
pub fn op_92(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 92");
}
pub fn op_93(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 93");
}
pub fn op_94(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 94");
}
pub fn op_95(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 95");
}
pub fn op_96(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 96");
}
pub fn op_97(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 97");
}
pub fn op_98(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 98");
}
pub fn op_99(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 99");
}
pub fn op_9a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9A");
}
pub fn op_9b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9B");
}
pub fn op_9c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9C");
}
pub fn op_9d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9D");
}
pub fn op_9e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9E");
}
pub fn op_9f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 9F");
}
pub fn op_a0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A0");
}
pub fn op_a1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A1");
}
pub fn op_a2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A2");
}
pub fn op_a3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A3");
}
pub fn op_a4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A4");
}
pub fn op_a5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A5");
}
pub fn op_a6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A6");
}
pub fn op_a7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A7");
}
pub fn op_a8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A8");
}
pub fn op_a9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode A9");
}
pub fn op_aa(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AA");
}
pub fn op_ab(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AB");
}
pub fn op_ac(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AC");
}
pub fn op_ad(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AD");
}
pub fn op_ae(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AE");
}
pub fn op_af(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode AF");
}
pub fn op_b0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B0");
}
pub fn op_b1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B1");
}
pub fn op_b2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B2");
}
pub fn op_b3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B3");
}
pub fn op_b4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B4");
}
pub fn op_b5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B5");
}
pub fn op_b6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B6");
}
pub fn op_b7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode B7");
}

pub fn op_b8(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[0].set(RegType::R16, val as u64);

            println!("MOV AX, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[0].set(RegType::R32, val as u64);

            println!("MOV EAX, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[0].set(RegType::R64, val);

            println!("MOV RAX, 0x{val:0>16X}");
            return false;
        }
    }

    false
}

pub fn op_b9(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[1].set(RegType::R16, val as u64);

            println!("MOV CX, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[1].set(RegType::R32, val as u64);

            println!("MOV ECX, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[1].set(RegType::R64, val);

            println!("MOV RCX, 0x{val:0>16X}");
            return false;
        }
    }

    false
}
pub fn op_ba(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[2].set(RegType::R16, val as u64);

            println!("MOV DX, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[2].set(RegType::R32, val as u64);

            println!("MOV EDX, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[2].set(RegType::R64, val);

            println!("MOV RDX, 0x{val:0>16X}");
            return false;
        }
    }

    false
}

pub fn op_bb(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[3].set(RegType::R16, val as u64);

            println!("MOV BX, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[3].set(RegType::R32, val as u64);

            println!("MOV EBX, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[3].set(RegType::R64, val);

            println!("MOV RBX, 0x{val:0>16X}");
            return false;
        }
    }

    false
}

pub fn op_bc(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[4].set(RegType::R16, val as u64);

            println!("MOV SP, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[4].set(RegType::R32, val as u64);

            println!("MOV ESP, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[4].set(RegType::R64, val);

            println!("MOV RSP, 0x{val:0>16X}");
            return false;
        }
    }

    false
}
pub fn op_bd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode BD");
}
pub fn op_be(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();

    match op {
        OperandSize::BIT16 => {
            let val: u16 = cpu.read_val16(ram);
            cpu.regs[6].set(RegType::R16, val as u64);

            println!("MOV SI, 0x{val:0>4X}");
        },
        OperandSize::BIT32 => {
            let val: u32 = cpu.read_val32(ram);
            cpu.regs[6].set(RegType::R32, val as u64);

            println!("MOV ESI, 0x{val:0>8X}");
        },
        OperandSize::BIT64 => {
            let val: u64 = cpu.read_val64(ram);
            cpu.regs[6].set(RegType::R64, val);

            println!("MOV RSI, 0x{val:0>16X}");
            return false;
        }
    }

    false
}
pub fn op_bf(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode BF");
}
pub fn op_c0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C0");
}

pub fn op_c1(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);

    OP_C1_TBL[modrm._reg as usize](cpu, ram, &modrm);

    false
}

pub fn op_c2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C2");
}
pub fn op_c3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C3");
}
pub fn op_c4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C4");
}
pub fn op_c5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C5");
}
pub fn op_c6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C6");
}
pub fn op_c7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C7");
}
pub fn op_c8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C8");
}
pub fn op_c9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode C9");
}
pub fn op_ca(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CA");
}
pub fn op_cb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CB");
}
pub fn op_cc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CC");
}
pub fn op_cd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CD");
}
pub fn op_ce(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CE");
}
pub fn op_cf(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode CF");
}
pub fn op_d0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D0");
}
pub fn op_d1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D1");
}
pub fn op_d2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D2");
}
pub fn op_d3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D3");
}
pub fn op_d4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D4");
}
pub fn op_d5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D5");
}
pub fn op_d6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D6");
}
pub fn op_d7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D7");
}
pub fn op_d8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D8");
}
pub fn op_d9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode D9");
}
pub fn op_da(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DA");
}
pub fn op_db(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DB");
}
pub fn op_dc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DC");
}
pub fn op_dd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DD");
}
pub fn op_de(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DE");
}
pub fn op_df(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode DF");
}
pub fn op_e0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E0");
}
pub fn op_e1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E1");
}
pub fn op_e2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E2");
}
pub fn op_e3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E3");
}
pub fn op_e4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E4");
}
pub fn op_e5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E5");
}
pub fn op_e6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E6");
}
pub fn op_e7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E7");
}
pub fn op_e8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode E8");
}

pub fn op_e9(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();
    if op == OperandSize::BIT16 {
        let jump_addr: i16 = cpu.read_val16(ram) as i16;
        let mut ip: u16 = cpu.ip.get(RegType::R16) as u16;
        
        ip = ip.wrapping_add_signed(jump_addr);
        cpu.ip.set(RegType::R16, ip as u64);

        println!("JMP 0x{jump_addr:0>4X}")
    } else {
        let jump_addr: i32 = cpu.read_val32(ram) as i32;
        let mut ip: u32 = cpu.ip.get(RegType::R16) as u32;
        
        ip = ip.wrapping_add_signed(jump_addr);
        cpu.ip.set(RegType::R16, ip as u64);

        println!("JMP 0x{jump_addr:0>8X}")
    }

    false
}

pub fn op_ea(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let op = cpu.deter_op_size();
    if op == OperandSize::BIT16 {
        let jump_addr: u16 = cpu.read_val16(ram);
        let selector:  u16 = cpu.read_val16(ram);

        cpu.ip.val = jump_addr as u64;

        let did_shift = cpu.check_bit_mode_shift();

        println!("JMPF 0x{selector:0>4X}:0x{jump_addr:0>4X}");
        if did_shift {
            cpu.setup_cs_seg(ram, selector);
            println!("SHIFTED INTO NEXT MODE");
        }
    } else {
        let jump_addr: u32 = cpu.read_val32(ram);
        let selector:  u16 = cpu.read_val16(ram);

        cpu.ip.val = jump_addr as u64;

        let did_shift = cpu.check_bit_mode_shift();

        println!("JMPF 0x{selector:0>4X}:0x{jump_addr:0>8X}");
        if did_shift {
            cpu.setup_cs_seg(ram, selector);
            println!("SHIFTED INTO NEXT MODE");
        }
    }

    false
}

pub fn op_eb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode EB");
}
pub fn op_ec(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode EC");
}
pub fn op_ed(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode ED");
}
pub fn op_ee(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode EE");
}
pub fn op_ef(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode EF");
}
pub fn op_f0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F0");
}
pub fn op_f1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F1");
}
pub fn op_f2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F2");
}
pub fn op_f3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F3");
}
pub fn op_f4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F4");
}
pub fn op_f5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F5");
}
pub fn op_f6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F6");
}
pub fn op_f7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F7");
}
pub fn op_f8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F8");
}
pub fn op_f9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode F9");
}
pub fn op_fa(cpu: &mut CPU, _ram: &mut RAM) -> bool {
    if cpu.is_16bit_mode() {
        Flag::set_flag(&mut cpu.flags, Flag::INT_FLAG, false);
    } else if Flag::get_iopl_flag(cpu.flags) as u16 >= (cpu.cs().selector & 0b11) {
        Flag::set_flag(&mut cpu.flags, Flag::INT_FLAG, false);
    } else if CR4::get_flag(cpu.cr_regs[4], CR4::PROTMODE_VINT) {
        Flag::set_flag(&mut cpu.flags, Flag::VINT_FLAG, false);
    }

    println!("CLI");

    false
}
pub fn op_fb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode FB");
}
pub fn op_fc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode FC");
}
pub fn op_fd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode FD");
}
pub fn op_fe(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode FE");
}
pub fn op_ff(cpu: &mut CPU, ram: &mut RAM) -> bool{
    let modrm: ModRM = cpu.get_modrm(ram, RegType::R32);

    OP_FF_TBL[modrm._reg as usize](cpu, ram, &modrm);

    false
}
