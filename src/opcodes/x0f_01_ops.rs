use crate::{opcodes::std_ops::OpFuncRet, ram::RAM, reg::{ModRM, RegType}, x64::{AddressSize, CPU, InfoType}};

fn op_0f_01_0(_cpu: &mut CPU, _ram: &mut RAM, _modrm: &ModRM) -> bool {
    todo!("\n\nImplement opcode 0F 01 /0");
}

fn lidt(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) {
    let mut new_modrm: ModRM = modrm.clone();
    let ad_size: AddressSize = cpu.deter_ad_size();

    match ad_size {
        AddressSize::BIT16 => {
            cpu.get_ptr16(&mut new_modrm);
            new_modrm.rm_type = if modrm._rm >= 4 { Some(RegType::R16) } else { None };
            new_modrm.sib.idx_type  = Some(RegType::R16);
            new_modrm.sib.base_type = Some(RegType::R16);
        },
        AddressSize::BIT32 => {
            new_modrm.rm_type = Some(RegType::R32);
            new_modrm.rm = modrm._rm;
        },
        AddressSize::BIT64 => {
            new_modrm.rm_type = Some(RegType::R64);
            new_modrm.rm = modrm._rm;
        }
    }

    let mut _disp: u32 = 0;
    let addr = cpu.get_modrm_ptr(ram, &new_modrm, &mut _disp);
    
    match ad_size {
        AddressSize::BIT16 => {
            cpu.idtr.size = cpu.get_val16(ram, addr);
            cpu.idtr.addr = (cpu.get_val32(ram, addr + 2) & 0x00FFFFFF) as u64;
        },
        AddressSize::BIT32 => {
            cpu.idtr.size = cpu.get_val16(ram, addr);
            cpu.idtr.addr = cpu.get_val32(ram, addr + 2) as u64;
        },
        AddressSize::BIT64 => {
            cpu.idtr.size = cpu.get_val16(ram, addr);
            cpu.idtr.addr = cpu.get_val64(ram, addr + 2);
        }
    }
}

fn op_0f_01_1(cpu: &mut CPU, ram: &mut RAM, modrm: &ModRM) -> bool {
    match modrm._reg {
        3 => lidt(cpu, ram, modrm),
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