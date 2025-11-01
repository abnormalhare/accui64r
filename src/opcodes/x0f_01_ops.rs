use crate::{opcodes::std_ops::{OpFuncRet}, ram::RAM, reg::ModRM, x64::CPU};

fn op_0f_01_0(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /0");
}

fn lidt(cpu: &mut CPU, ram: &mut RAM, rm: u8) {
    
}

fn op_0f_01_1(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) -> bool {
    match modrm._reg {
        3 => lidt(cpu, ram, modrm._rm),
        _ => {}
    }
    false
}

fn op_0f_01_2(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /2");
}
fn op_0f_01_3(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /3");
}
fn op_0f_01_4(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /4");
}
fn op_0f_01_5(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /5");
}
fn op_0f_01_6(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /6");
}
fn op_0f_01_7(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /7");
}

pub const OP_0F_01_TBL: [OpFuncRet; 8] = [
    op_0f_01_0, op_0f_01_1, op_0f_01_2, op_0f_01_3,
    op_0f_01_4, op_0f_01_5, op_0f_01_6, op_0f_01_7,
];