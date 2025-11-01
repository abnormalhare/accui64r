use std::env;

pub mod alu;
pub mod debug;
pub mod opcodes;
pub mod ram;
pub mod reg;
pub mod x64;

fn ret_if_next_arg_none(arg: Option<(usize, String)>) {
    if arg == None {
        panic!("USAGE: accui64.exe [FILENAME]");
    }
}

fn main() {
    let mut args = env::args().enumerate();
    
    ret_if_next_arg_none(args.next());

    let mut ram = ram::RAM::new();
    match ram.load(args.next().unwrap().1) {
        Ok(()) => {},
        Err(e) => {
            println!("Failed to load ram: {}", e);
            return;
        }
    }

    let mut cpu: x64::CPU = x64::CPU::new();
    cpu.run(&mut ram);
}
