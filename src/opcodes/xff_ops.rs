use crate::{debug::{OpOrder, dbp}, opcodes::std_ops::OpFunc, ram::RAM, reg::{ModRM}, x64::{CPU, OperandSize}};

fn op_ff_0(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /0");
}
fn op_ff_1(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /1");
}
fn op_ff_2(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /2");
}
fn op_ff_3(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /3");
}
fn op_ff_4(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) {
    let op = cpu.deter_op_size();
    let reg_type = op.to_reg_type();
    let mut disp: u32 = 0;

    if modrm.rm_is_reg() {
        let jump_addr: u64 = cpu.regs[modrm.rm as usize].get(modrm.rm_type.unwrap());
        
        cpu.ip.set(reg_type, jump_addr);
    } else {
        let addr: u64 = cpu.get_modrm_ptr(ram, &modrm, &mut disp);
        let jump_addr: u64 =
            match op {
                OperandSize::BIT16 => cpu.get_val16(ram, addr) as u64,
                OperandSize::BIT32 => cpu.get_val32(ram, addr) as u64,
                OperandSize::BIT64 => cpu.get_val64(ram, addr),
            };
        
        cpu.ip.set(reg_type, jump_addr);
    }

    dbp(cpu, "JMP", modrm, disp, 0, OpOrder::RM);
}
fn op_ff_5(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /5");
}
fn op_ff_6(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /6");
}
fn op_ff_7(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) {
    todo!("\n\nImplement opcode FF /7");
}

pub const OP_FF_TBL: [OpFunc; 8] = [
    op_ff_0, op_ff_1, op_ff_2, op_ff_3,
    op_ff_4, op_ff_5, op_ff_6, op_ff_7,
];