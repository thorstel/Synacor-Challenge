use std::env;
use std::error::Error;
use std::fs;
use std::io;

use synacor_challenge::*;

fn main() -> Result<(), Box<dyn Error>> {
    let prog = parse_input("challenge.bin")?;
    let mut vm = VirtualMachine::new(&prog);

    let mut args = env::args();
    args.next();
    if let Some(file) = args.next() {
        let save_game = fs::read_to_string(file)?;
        for cmd in save_game.lines() {
            cmd.trim().chars().for_each(|c| vm.input.push_back(c as u16));
            vm.input.push_back('\n' as u16);
        }
    }

    let mut trace = Vec::new();
    loop {
        if vm.run() { break; }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "abort" { break; }
        line.trim().chars().for_each(|c| vm.input.push_back(c as u16));
        vm.input.push_back('\n' as u16);
        trace.push(line);
    }

    println!("=== INPUT TRACE ===");
    for cmd in trace.iter() {
        print!("{}", cmd);
    }

    Ok(())
}
