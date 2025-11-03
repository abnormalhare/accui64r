use crate::{alu, debug::{OpOrder, dbp}, opcodes::std_ops::OpFunc, ram::RAM, reg::{ModRM, RegType}, x64::CPU};

fn op_83_0(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /0");
}
fn op_83_1(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /1");
}
fn op_83_2(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /2");
}
fn op_83_3(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /3");
}
fn op_83_4(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /3");
}
fn op_83_5(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /5");
}
fn op_83_6(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode 83 /6");
}
fn op_83_7(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) {
    let mut disp: u32 = 0;
    let reg_type: RegType = modrm.reg_type.unwrap();
    let val = cpu.read_val8(ram);

    if modrm.rm_is_reg() {
        let reg = cpu.regs[modrm.rm as usize].get(reg_type);

        alu::sub(cpu, reg_type, reg, val.into());
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        let reg = cpu.get(ram, addr);

        alu::sub(cpu, reg_type, reg.into(), val.into());
    }

    dbp(cpu, "CMP", &modrm, disp, 0, OpOrder::RM_VAL);
}

pub const OP_83_TBL: [OpFunc; 8] = [
    op_83_0, op_83_1, op_83_2, op_83_3,
    op_83_4, op_83_5, op_83_6, op_83_7,
];