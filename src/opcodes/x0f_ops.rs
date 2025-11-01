use crate::{debug::get_reg_name, opcodes::x0f_01_ops::OP_0F_01_TBL, ram::RAM, reg::{ModRM, Reg, RegType}, x64::CPU};

pub fn op_0f_00(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 00");
}

pub fn op_0f_01(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let val: u8 = cpu.read_code(ram);
    let modrm: ModRM = ModRM::new(val.into());

    OP_0F_01_TBL[modrm._reg as usize](cpu, ram, &modrm)
}

pub fn op_0f_02(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 02");
}
pub fn op_0f_03(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 03");
}
pub fn op_0f_04(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 04");
}
pub fn op_0f_05(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 05");
}
pub fn op_0f_06(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 06");
}
pub fn op_0f_07(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 07");
}
pub fn op_0f_08(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 08");
}
pub fn op_0f_09(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 09");
}
pub fn op_0f_0a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0A");
}
pub fn op_0f_0b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0B");
}
pub fn op_0f_0c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0C");
}
pub fn op_0f_0d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0D");
}
pub fn op_0f_0e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0E");
}
pub fn op_0f_0f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 0F");
}
pub fn op_0f_10(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 10");
}
pub fn op_0f_11(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 11");
}
pub fn op_0f_12(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 12");
}
pub fn op_0f_13(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 13");
}
pub fn op_0f_14(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 14");
}
pub fn op_0f_15(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 15");
}
pub fn op_0f_16(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 16");
}
pub fn op_0f_17(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 17");
}
pub fn op_0f_18(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 18");
}
pub fn op_0f_19(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 19");
}
pub fn op_0f_1a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1A");
}
pub fn op_0f_1b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1B");
}
pub fn op_0f_1c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1C");
}
pub fn op_0f_1d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1D");
}
pub fn op_0f_1e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1E");
}
pub fn op_0f_1f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 1F");
}
pub fn op_0f_20(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 20");
}
pub fn op_0f_21(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 21");
}

pub fn op_0f_22(cpu: &mut CPU, ram: &mut RAM) -> bool {
    let val: u8 = cpu.read_code(ram);
    let modrm: ModRM = ModRM::new(val as u64);
    let reg_type: RegType = if cpu.is_64bit_mode() { RegType::R64 } else { RegType::R32 };

    let src: &Reg     = &    cpu.regs   [modrm._rm  as usize];
    let dst: &mut u64 = &mut cpu.cr_regs[modrm._reg as usize];

    *dst = src.get(reg_type);

    println!("MOV CR{}, {}", modrm._reg, get_reg_name(modrm._rm, reg_type));

    false
}

pub fn op_0f_23(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 23");
}
pub fn op_0f_24(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 24");
}
pub fn op_0f_25(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 25");
}
pub fn op_0f_26(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 26");
}
pub fn op_0f_27(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 27");
}
pub fn op_0f_28(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 28");
}
pub fn op_0f_29(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 29");
}
pub fn op_0f_2a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2A");
}
pub fn op_0f_2b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2B");
}
pub fn op_0f_2c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2C");
}
pub fn op_0f_2d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2D");
}
pub fn op_0f_2e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2E");
}
pub fn op_0f_2f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 2F");
}
pub fn op_0f_30(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 30");
}
pub fn op_0f_31(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 31");
}
pub fn op_0f_32(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 32");
}
pub fn op_0f_33(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 33");
}
pub fn op_0f_34(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 34");
}
pub fn op_0f_35(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 35");
}
pub fn op_0f_36(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 36");
}
pub fn op_0f_37(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 37");
}
pub fn op_0f_38(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 38");
}
pub fn op_0f_39(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 39");
}
pub fn op_0f_3a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3A");
}
pub fn op_0f_3b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3B");
}
pub fn op_0f_3c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3C");
}
pub fn op_0f_3d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3D");
}
pub fn op_0f_3e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3E");
}
pub fn op_0f_3f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 3F");
}
pub fn op_0f_40(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 40");
}
pub fn op_0f_41(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 41");
}
pub fn op_0f_42(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 42");
}
pub fn op_0f_43(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 43");
}
pub fn op_0f_44(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 44");
}
pub fn op_0f_45(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 45");
}
pub fn op_0f_46(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 46");
}
pub fn op_0f_47(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 47");
}
pub fn op_0f_48(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 48");
}
pub fn op_0f_49(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 49");
}
pub fn op_0f_4a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4A");
}
pub fn op_0f_4b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4B");
}
pub fn op_0f_4c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4C");
}
pub fn op_0f_4d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4D");
}
pub fn op_0f_4e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4E");
}
pub fn op_0f_4f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 4F");
}
pub fn op_0f_50(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 50");
}
pub fn op_0f_51(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 51");
}
pub fn op_0f_52(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 52");
}
pub fn op_0f_53(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 53");
}
pub fn op_0f_54(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 54");
}
pub fn op_0f_55(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 55");
}
pub fn op_0f_56(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 56");
}
pub fn op_0f_57(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 57");
}
pub fn op_0f_58(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 58");
}
pub fn op_0f_59(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 59");
}
pub fn op_0f_5a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5A");
}
pub fn op_0f_5b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5B");
}
pub fn op_0f_5c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5C");
}
pub fn op_0f_5d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5D");
}
pub fn op_0f_5e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5E");
}
pub fn op_0f_5f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 5F");
}
pub fn op_0f_60(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 60");
}
pub fn op_0f_61(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 61");
}
pub fn op_0f_62(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 62");
}
pub fn op_0f_63(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 63");
}
pub fn op_0f_64(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 64");
}
pub fn op_0f_65(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 65");
}
pub fn op_0f_66(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 66");
}
pub fn op_0f_67(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 67");
}
pub fn op_0f_68(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 68");
}
pub fn op_0f_69(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 69");
}
pub fn op_0f_6a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6A");
}
pub fn op_0f_6b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6B");
}
pub fn op_0f_6c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6C");
}
pub fn op_0f_6d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6D");
}
pub fn op_0f_6e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6E");
}
pub fn op_0f_6f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 6F");
}
pub fn op_0f_70(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 70");
}
pub fn op_0f_71(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 71");
}
pub fn op_0f_72(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 72");
}
pub fn op_0f_73(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 73");
}
pub fn op_0f_74(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 74");
}
pub fn op_0f_75(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 75");
}
pub fn op_0f_76(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 76");
}
pub fn op_0f_77(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 77");
}
pub fn op_0f_78(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 78");
}
pub fn op_0f_79(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 79");
}
pub fn op_0f_7a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7A");
}
pub fn op_0f_7b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7B");
}
pub fn op_0f_7c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7C");
}
pub fn op_0f_7d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7D");
}
pub fn op_0f_7e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7E");
}
pub fn op_0f_7f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 7F");
}
pub fn op_0f_80(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 80");
}
pub fn op_0f_81(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 81");
}
pub fn op_0f_82(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 82");
}
pub fn op_0f_83(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 83");
}
pub fn op_0f_84(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 84");
}
pub fn op_0f_85(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 85");
}
pub fn op_0f_86(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 86");
}
pub fn op_0f_87(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 87");
}
pub fn op_0f_88(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 88");
}
pub fn op_0f_89(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 88");
}
pub fn op_0f_8a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8A");
}
pub fn op_0f_8b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8B");
}
pub fn op_0f_8c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8C");
}
pub fn op_0f_8d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8D");
}
pub fn op_0f_8e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8E");
}
pub fn op_0f_8f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 8F");
}
pub fn op_0f_90(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 90");
}
pub fn op_0f_91(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 91");
}
pub fn op_0f_92(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 92");
}
pub fn op_0f_93(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 93");
}
pub fn op_0f_94(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 94");
}
pub fn op_0f_95(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 95");
}
pub fn op_0f_96(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 96");
}
pub fn op_0f_97(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 97");
}
pub fn op_0f_98(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 98");
}
pub fn op_0f_99(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 99");
}
pub fn op_0f_9a(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9A");
}
pub fn op_0f_9b(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9B");
}
pub fn op_0f_9c(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9C");
}
pub fn op_0f_9d(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9D");
}
pub fn op_0f_9e(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9E");
}
pub fn op_0f_9f(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F 9F");
}
pub fn op_0f_a0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A0");
}
pub fn op_0f_a1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A1");
}
pub fn op_0f_a2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A2");
}
pub fn op_0f_a3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A3");
}
pub fn op_0f_a4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A4");
}
pub fn op_0f_a5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A5");
}
pub fn op_0f_a6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A6");
}
pub fn op_0f_a7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A7");
}
pub fn op_0f_a8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A8");
}
pub fn op_0f_a9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F A9");
}
pub fn op_0f_aa(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AA");
}
pub fn op_0f_ab(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AB");
}
pub fn op_0f_ac(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AC");
}
pub fn op_0f_ad(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AD");
}
pub fn op_0f_ae(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AE");
}
pub fn op_0f_af(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F AF");
}
pub fn op_0f_b0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B0");
}
pub fn op_0f_b1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B1");
}
pub fn op_0f_b2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B2");
}
pub fn op_0f_b3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B3");
}
pub fn op_0f_b4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B4");
}
pub fn op_0f_b5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B5");
}
pub fn op_0f_b6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B6");
}
pub fn op_0f_b7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B7");
}
pub fn op_0f_b8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B8");
}
pub fn op_0f_b9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F B9");
}
pub fn op_0f_ba(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BA");
}
pub fn op_0f_bb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BB");
}
pub fn op_0f_bc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BC");
}
pub fn op_0f_bd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BD");
}
pub fn op_0f_be(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BE");
}
pub fn op_0f_bf(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F BF");
}
pub fn op_0f_c0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C0");
}
pub fn op_0f_c1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C1");
}
pub fn op_0f_c2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C2");
}
pub fn op_0f_c3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C3");
}
pub fn op_0f_c4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C4");
}
pub fn op_0f_c5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C5");
}
pub fn op_0f_c6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C6");
}
pub fn op_0f_c7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C7");
}
pub fn op_0f_c8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C8");
}
pub fn op_0f_c9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F C9");
}
pub fn op_0f_ca(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CA");
}
pub fn op_0f_cb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CB");
}
pub fn op_0f_cc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CC");
}
pub fn op_0f_cd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CD");
}
pub fn op_0f_ce(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CE");
}
pub fn op_0f_cf(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F CF");
}
pub fn op_0f_d0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D0");
}
pub fn op_0f_d1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D1");
}
pub fn op_0f_d2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D2");
}
pub fn op_0f_d3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D3");
}
pub fn op_0f_d4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D4");
}
pub fn op_0f_d5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D5");
}
pub fn op_0f_d6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D6");
}
pub fn op_0f_d7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D7");
}
pub fn op_0f_d8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D8");
}
pub fn op_0f_d9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F D9");
}
pub fn op_0f_da(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DA");
}
pub fn op_0f_db(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DB");
}
pub fn op_0f_dc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DC");
}
pub fn op_0f_dd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DD");
}
pub fn op_0f_de(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DE");
}
pub fn op_0f_df(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F DF");
}
pub fn op_0f_e0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E0");
}
pub fn op_0f_e1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E1");
}
pub fn op_0f_e2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E2");
}
pub fn op_0f_e3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E3");
}
pub fn op_0f_e4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E4");
}
pub fn op_0f_e5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E5");
}
pub fn op_0f_e6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E6");
}
pub fn op_0f_e7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E7");
}
pub fn op_0f_e8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E8");
}
pub fn op_0f_e9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F E9");
}
pub fn op_0f_ea(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F EA");
}
pub fn op_0f_eb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F EB");
}
pub fn op_0f_ec(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F EC");
}
pub fn op_0f_ed(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F ED");
}
pub fn op_0f_ee(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F EE");
}
pub fn op_0f_ef(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F EF");
}
pub fn op_0f_f0(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F0");
}
pub fn op_0f_f1(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F1");
}
pub fn op_0f_f2(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F2");
}
pub fn op_0f_f3(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F3");
}
pub fn op_0f_f4(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F4");
}
pub fn op_0f_f5(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F5");
}
pub fn op_0f_f6(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F6");
}
pub fn op_0f_f7(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F7");
}
pub fn op_0f_f8(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F8");
}
pub fn op_0f_f9(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F F9");
}
pub fn op_0f_fa(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F FA");
}
pub fn op_0f_fb(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F FB");
}
pub fn op_0f_fc(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F FC");
}
pub fn op_0f_fd(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F FD");
}
pub fn op_0f_fe(_cpu: &mut CPU, _ram: &mut RAM) -> bool {
    todo!("\n\nImplement opcode 0F FE");
}
pub fn op_0f_ff(_cpu: &mut CPU, _ram: &mut RAM) -> bool{
    todo!("\n\nImplement opcode 0F FF");
}
