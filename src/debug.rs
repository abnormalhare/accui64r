use crate::{reg::{ModRM, RegType}, x64::CPU};

static R8_NAMES: [&str; 0x10] = [
    "AL",   "CL",   "DL",   "BL",
    "SPL",  "BPL",  "SIL",  "DIL",
    "R8B",  "R9B",  "R10B", "R11B",
    "R12B", "R13B", "R14B", "R15B",
];

static R8H_NAMES: [&str; 0x10] = [
    "AH",    "CH",     "DH",    "BH",
    "", "", "", "", "", "", "", "", "", "", "", ""
];

static R16_NAMES: [&str; 0x10] = [
    "AX",   "CX",   "DX",   "BX",
    "SP",   "BP",   "SI",   "DI",
    "R8W",  "R9W",  "R10W", "R11W",
    "R12W", "R13W", "R14W", "R15W",
];

static R32_NAMES: [&str; 0x10] = [
    "EAX",   "ECX", "EDX",  "EBX",
    "ESP",  "EBP",  "ESI",  "EDI",
    "R8D",  "R9D",  "R10D", "R11D",
    "R12D", "R13D", "R14D", "R15D",
];

static R64_NAMES: [&str; 0x10] = [
    "RAX", "RCX", "RDX", "RBX",
    "RSP", "RBP", "RSI", "RDI",
    "R8",  "R9",  "R10", "R11",
    "R12", "R13", "R14", "R15",
];

static ST_NAMES: [&str; 6] = ["ES", "CS", "SS", "DS", "FS", "GS"];

static MM_NAMES: [&str; 0x10] = [
    "MM0",  "MM1",  "MM2",  "MM3",
    "MM4",  "MM5",  "MM6",  "MM7",
    "MM8",  "MM9",  "MM10", "MM11",
    "MM12", "MM13", "MM14", "MM15",
];

static XMM_NAMES: [&str; 0x10] = [
    "XMM0",  "XMM1",  "XMM2",  "XMM3",
    "XMM4",  "XMM5",  "XMM6",  "XMM7",
    "XMM8",  "XMM9",  "XMM10", "XMM11",
    "XMM12", "XMM13", "XMM14", "XMM15",
];

pub fn get_reg_name(idx: u8, reg_type: RegType) -> &'static str {
    let idx = idx as usize;
    match reg_type {
        RegType::R8  => R8_NAMES [idx],
        RegType::R8H => R8H_NAMES[idx],
        RegType::R16 => R16_NAMES[idx],
        RegType::R32 => R32_NAMES[idx],
        RegType::R64 => R64_NAMES[idx],
        
        RegType::ST => ST_NAMES[idx],
        RegType::MM => MM_NAMES[idx],
        RegType::XMM => XMM_NAMES[idx],
    }
}

pub fn get_reg_ptr_name(reg_type: RegType) -> &'static str {
    match reg_type {
        RegType::R8 | RegType::R8H => "BYTE PTR",
        RegType::R16 => "WORD PTR",
        RegType::R32 => "DWORD PTR",
        RegType::R64 => "QWORD PTR",
        
        RegType::ST => "",
        RegType::MM => "MMWORD PTR",
        RegType::XMM => "XMMWORD PTR",
    }
}

fn dbp_mem(cpu: &CPU, modrm: &ModRM, disp: u32) -> String {
    let mut ret: String = String::new();
    

    if modrm._mod == 3 {
        let rm_type  = modrm.rm_type.unwrap();

        ret += get_reg_name(modrm.rm, rm_type);
        return ret;
    }

    let segidx = cpu.get_seg_override_idx();
    if !segidx.is_none() {
        ret += get_reg_name(segidx.unwrap(), RegType::ST);
        ret += ":"
    }

    if !modrm.reg_type.is_none() {
        ret += get_reg_ptr_name(modrm.reg_type.unwrap());
    }
    
    if segidx.is_none() {
        ret += " ";
    }
    ret += "[";

    if !modrm.should_use_sib() {
        if !modrm.rm_type.is_none() {
            let rm_type  = modrm.rm_type.unwrap();

            ret += get_reg_name(modrm.rm, rm_type);
        }
    } else {
        if !modrm.sib.idx_type.is_none() {
            let idx_type = modrm.sib.idx_type.unwrap();

            ret += get_reg_name(modrm.sib.idx, idx_type);
            ret += &format!("* {}", modrm.sib.mul);
        }

        if !modrm.sib.base_type.is_none() {
            if !modrm.sib.idx_type.is_none() {
                ret += " + ";
            }

            let base_type = modrm.sib.base_type.unwrap();

            ret += get_reg_name(modrm.sib.base, base_type);
        }
    }

    if modrm.disp != 0 {
        if !modrm.rm_type.is_none() || !modrm.sib.idx_type.is_none() || !modrm.sib.base_type.is_none() {
            ret += " + ";
        }

        match modrm.disp {
            1 => ret += &format!("0x{disp:0>2X}"),
            2 => ret += &format!("0x{disp:0>4X}"),
            4 => ret += &format!("0x{disp:0>8X}"),
            _ => {},
        }
    }

    ret += "]";

    ret
}

fn dbp_reg(modrm: &ModRM) -> String {
    get_reg_name(modrm._reg, modrm.reg_type.unwrap()).to_string()
}

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum OpOrder {
    RM_R,
    RM_VAL,
    R_RM,
    R_VAL,
    RM,
}

pub fn dbp(cpu: &CPU, name: &str, modrm: &ModRM, disp: u32, val: u64, order: OpOrder) {
    let mut out: String = name.to_string();
    out += " ";

    match order {
        OpOrder::RM_R => {
            out += &dbp_mem(cpu, modrm, disp);
            out += ", ";
            out += &dbp_reg(modrm);
        },
        OpOrder::R_RM => {
            out += &dbp_reg(modrm);
            out += ", ";
            out += &dbp_mem(cpu, modrm, disp);
        },
        OpOrder::RM_VAL => {
            out += &dbp_mem(cpu, modrm, disp);
            out += ", ";
            out += &format!("{val}");
        },
        OpOrder::R_VAL => {
            out += &dbp_reg(modrm);
            out += ", ";
            out += &format!("{val}");
        },
        OpOrder::RM => {
            out += &dbp_mem(cpu, modrm, disp);
        }
    }

    println!("{}", out);
}