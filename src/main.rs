use std::error::Error;
use std::io;

use synacor_challenge::*;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = parse_input("challenge.bin")?;
    let mut vm = VirtualMachine::new(&prog);
    loop {
        if vm.run() { break; }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().for_each(|c| vm.input.push_back(c as u16));
        vm.input.push_back('\n' as u16);
    }
    Ok(())
}
