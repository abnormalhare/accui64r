use crate::{alu, debug::{OpOrder, dbp}, opcodes::std_ops::OpFunc, ram::RAM, reg::{ModRM, RegType}, x64::CPU};

fn op_c1_0(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /0");
}
fn op_c1_1(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /1");
}
fn op_c1_2(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /2");
}
fn op_c1_3(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /3");
}

fn op_c1_4(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) {
    let reg_type: RegType = modrm.reg_type.unwrap();
    let mut disp: u32 = 0;
    let val: u64 = cpu.read_code(ram) as u64;

    if modrm.rm_is_reg() {
        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::shl(cpu, reg_type, dst, val);

        cpu.regs[modrm.rm as usize].set(reg_type, res);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);

        let dst = cpu.regs[modrm.rm as usize].get(reg_type);
        let res = alu::shl(cpu, reg_type, dst, val);
        
        cpu.write(ram, addr as usize, res, cpu.get_size(reg_type));
    }

    dbp("SHL", modrm, disp, val, OpOrder::RM_VAL);
}

fn op_c1_5(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /5");
}
fn op_c1_6(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /6");
}
fn op_c1_7(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode C1 /7");
}

pub const OP_C1_TBL: [OpFunc; 8] = [
    op_c1_0, op_c1_1, op_c1_2, op_c1_3,
    op_c1_4, op_c1_5, op_c1_6, op_c1_7,
];