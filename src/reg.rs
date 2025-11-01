use crate::{ram, x64::{self, get_mask}};

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
            RegType::R8H => x64::get_mask(self.val, 15, 8).into(),
            _ => self.val.into(),
        }
    }

    pub fn set(&mut self, reg_type: RegType, val: u64) {
        match reg_type {
            RegType::R8H => {
                self.val &= !0xFF00;
                self.val |= val << 8;
            },
            _ => self.val = val.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub union XMMReg {
    pub valn: [u64; 8],
    pub valf: [f64; 8],
}

#[derive(Clone, Copy)]
pub struct SegReg {
    pub selector: u16,
    pub base: u32,
    pub limit: u32,
    pub attr: u16,
}

#[derive(Clone, Copy)]
pub struct DataTableReg {
    pub size: u16,
    pub addr: u64,
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
}