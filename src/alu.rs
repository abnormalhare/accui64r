use crate::{reg::{Flag, RegType, get_size}, x64::CPU};

fn get_parity(val: u8) -> bool {
    let mut res: u8 = val;
    res ^= res >> 4;
    res ^= res >> 2;
    res ^= res >> 1;

    (res & 1) == 0
}

pub fn get_bit_mask(reg_type: RegType, bits: usize) -> u64 {
    if reg_type == RegType::R64 { u64::MAX } else { (1u64 << bits) - 1 }
}

pub fn shl(cpu: &mut CPU, reg_type: RegType, a: u64, b: u64) -> u64 {
    let (res, _) = a.overflowing_shl(b as u32 /* ! */);
    let bits: usize = get_size(reg_type) * 8;
    let mask: u64 = get_bit_mask(reg_type, bits);

    if b != 0 { // if shift == 0, carry unchanged
        let mask: u64 = 1u64 << (bits - 1);
        Flag::set_flag(&mut cpu.flags, Flag::CARRY_FLAG,  ((a << (b - 1)) & mask) != 0);
    }
    Flag::set_flag(&mut cpu.flags, Flag::PARITY_FLAG, get_parity(res as u8));
    Flag::set_flag(&mut cpu.flags, Flag::AUX_FLAG,    false);
    Flag::set_flag(&mut cpu.flags, Flag::ZERO_FLAG,   res == 0);
    Flag::set_flag(&mut cpu.flags, Flag::SIGN_FLAG,   ((res >> (bits - 1)) & 1) != 0);

    if b == 1 { // if shift != 1, overflow undefined
        let msb_o: u64 = (a   >> (bits - 1)) & 1;
        let msb_n: u64 = (res >> (bits - 1)) & 1;

        Flag::set_flag(&mut cpu.flags, Flag::OVER_FLAG,   (msb_o ^ msb_n) != 0);
    } else {
        Flag::set_flag(&mut cpu.flags, Flag::OVER_FLAG,   false);
    }

    res & mask
}

pub fn sub(cpu: &mut CPU, reg_type: RegType, a: u64, b: u64) -> u64 {
    let bits: usize = get_size(reg_type) * 8;
    let mask: u64 = get_bit_mask(reg_type, bits);
    let res: u64 = a.wrapping_sub(b) & mask;
    let topbit: u64 = 1u64 << (bits - 1);

    Flag::set_flag(&mut cpu.flags, Flag::CARRY_FLAG,  a < b);
    Flag::set_flag(&mut cpu.flags, Flag::PARITY_FLAG, get_parity(res as u8));
    Flag::set_flag(&mut cpu.flags, Flag::AUX_FLAG,    ((a ^ b ^ res) & 0x10) != 0);
    Flag::set_flag(&mut cpu.flags, Flag::ZERO_FLAG,   res == 0);
    Flag::set_flag(&mut cpu.flags, Flag::SIGN_FLAG,   ((res >> (bits - 1)) & 1) != 0);
    Flag::set_flag(&mut cpu.flags, Flag::OVER_FLAG,   ((a ^ b) & (a ^ res) & topbit) != 0);

    res
}

pub fn xor(cpu: &mut CPU, reg_type: RegType, a: u64, b: u64) -> u64 {
    let bits: usize = get_size(reg_type) * 8;
    let mask: u64 = get_bit_mask(reg_type, bits);
    let res: u64 = a ^ b;

    Flag::set_flag(&mut cpu.flags, Flag::CARRY_FLAG,  false);
    Flag::set_flag(&mut cpu.flags, Flag::PARITY_FLAG, get_parity(res as u8));
    Flag::set_flag(&mut cpu.flags, Flag::AUX_FLAG,    false);
    Flag::set_flag(&mut cpu.flags, Flag::ZERO_FLAG,   res == 0);
    Flag::set_flag(&mut cpu.flags, Flag::SIGN_FLAG,   ((res >> (bits - 1)) & 1) != 0);
    Flag::set_flag(&mut cpu.flags, Flag::OVER_FLAG,   false);

    res & mask
}