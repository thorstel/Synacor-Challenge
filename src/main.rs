use std::error::Error;

use synacor_challenge::*;

fn main() -> Result<(), Box<dyn Error>> {
    let prog   = self::parse_input("challenge.bin")?;
    let mut vm = VirtualMachine::new(&prog);
    vm.run();
    Ok(())
}
