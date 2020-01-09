use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const MEM_SIZE: usize = 32768;
const NUM_REGS: usize = 8;

pub struct VirtualMachine {
    pub input: VecDeque<u16>,
    ip:        usize,
    mem:       [u16; MEM_SIZE],
    reg:       [u16; NUM_REGS],
    stack:     Vec<u16>,
}

enum VMStatus {
    Success,
    PendingInput,
    Halt,
}

impl VirtualMachine {
    pub fn new(prog: &[u16]) -> VirtualMachine {
        assert!(prog.len() <= MEM_SIZE, "Program does not fit into memory!");
        let mut vm = VirtualMachine {
            ip:    0,
            mem:   [0; MEM_SIZE],
            reg:   [0; NUM_REGS],
            stack: Vec::new(),
            input: VecDeque::new(),
        };
        for (i, &v) in prog.iter().enumerate() {
            vm.mem[i] = v;
        }
        vm
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.exec_op() {
                VMStatus::Success      => (),
                VMStatus::Halt         => break true,
                VMStatus::PendingInput => break false,
            }
        }
    }

    fn exec_op(&mut self) -> VMStatus {
        match self.mem[self.ip] {
            0  => VMStatus::Halt,
            1  => self.op_set(),
            2  => self.op_push(),
            3  => self.op_pop(),
            4  => self.op_eq(),
            5  => self.op_gt(),
            6  => self.op_jmp(),
            7  => self.op_jt(),
            8  => self.op_jf(),
            9  => self.op_add(),
            10 => self.op_mult(),
            11 => self.op_mod(),
            12 => self.op_and(),
            13 => self.op_or(),
            14 => self.op_not(),
            15 => self.op_rmem(),
            16 => self.op_wmem(),
            17 => self.op_call(),
            18 => self.op_ret(),
            19 => self.op_out(),
            20 => self.op_in(),
            21 => self.op_noop(),
            _  => panic!("Invalid Opcode!"),
        }
    }

    fn op_set(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        self.set_value(a, b);
        self.ip += 3;
        VMStatus::Success
    }

    fn op_push(&mut self) -> VMStatus {
        let a = self.get_value(self.ip + 1);
        self.stack.push(a);
        self.ip += 2;
        VMStatus::Success
    }

    fn op_pop(&mut self) -> VMStatus {
        let a   = self.mem[self.ip + 1];
        let val = self.stack.pop().unwrap();
        self.set_value(a, val);
        self.ip += 2;
        VMStatus::Success
    }

    fn op_eq(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, if b == c { 1 } else { 0 });
        self.ip += 4;
        VMStatus::Success
    }

    fn op_gt(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, if b > c { 1 } else { 0 });
        self.ip += 4;
        VMStatus::Success
    }

    fn op_jmp(&mut self) -> VMStatus {
        let a = self.get_value(self.ip + 1) as usize;
        self.ip = a;
        VMStatus::Success
    }

    fn op_jt(&mut self) -> VMStatus {
        let a = self.get_value(self.ip + 1);
        let b = self.get_value(self.ip + 2) as usize;
        if a != 0 {
            self.ip = b;
        } else {
            self.ip += 3;
        }
        VMStatus::Success
    }

    fn op_jf(&mut self) -> VMStatus {
        let a = self.get_value(self.ip + 1);
        let b = self.get_value(self.ip + 2) as usize;
        if a == 0 {
            self.ip = b;
        } else {
            self.ip += 3;
        }
        VMStatus::Success
    }

    fn op_add(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, (b + c) % 32768);
        self.ip += 4;
        VMStatus::Success
    }

    fn op_mult(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2) as u64;
        let c = self.get_value(self.ip + 3) as u64;
        let result = ((b * c) % 32768) as u16;
        self.set_value(a, result);
        self.ip += 4;
        VMStatus::Success
    }

    fn op_mod(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, b % c);
        self.ip += 4;
        VMStatus::Success
    }

    fn op_and(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, b & c);
        self.ip += 4;
        VMStatus::Success
    }

    fn op_or(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        let c = self.get_value(self.ip + 3);
        self.set_value(a, b | c);
        self.ip += 4;
        VMStatus::Success
    }

    fn op_not(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2);
        self.set_value(a, !b & 0x7FFF);
        self.ip += 3;
        VMStatus::Success
    }

    fn op_rmem(&mut self) -> VMStatus {
        let a = self.mem[self.ip + 1];
        let b = self.get_value(self.ip + 2) as usize;
        self.set_value(a, self.mem[b]);
        self.ip += 3;
        VMStatus::Success
    }

    fn op_wmem(&mut self) -> VMStatus {
        let a = self.get_value(self.ip + 1) as usize;
        let b = self.get_value(self.ip + 2);
        self.mem[a]  = b;
        self.ip     += 3;
        VMStatus::Success
    }

    fn op_call(&mut self) -> VMStatus {
        let a    = self.get_value(self.ip + 1) as usize;
        let next = (self.ip + 2) as u16;
        self.stack.push(next);
        self.ip = a;
        VMStatus::Success
    }

    fn op_ret(&mut self) -> VMStatus {
        if self.stack.is_empty() {
            VMStatus::Halt
        } else {
            let addr = self.stack.pop().unwrap() as usize;
            self.ip = addr;
            VMStatus::Success
        }
    }

    fn op_out(&mut self) -> VMStatus {
        let a = char::from(self.get_value(self.ip + 1) as u8);
        print!("{}", a); // TODO?
        self.ip += 2;
        VMStatus::Success
    }

    fn op_in(&mut self) -> VMStatus {
        if self.input.is_empty() {
            VMStatus::PendingInput
        } else {
            let a   = self.mem[self.ip + 1];
            let val = self.input.pop_front().unwrap();
            self.set_value(a, val);
            self.ip += 2;
            VMStatus::Success
        }
    }

    fn op_noop(&mut self) -> VMStatus {
        self.ip += 1;
        VMStatus::Success
    }

    fn get_value(&self, addr: usize) -> u16 {
        let number = self.mem[addr];
        match number {
            0..=32767     => number,
            32768..=32775 => self.reg[self.get_reg(number)],
            32776..=65535 => panic!("Invalid Number!"),
        }
    }

    fn set_value(&mut self, addr: u16, value: u16) {
        match addr {
            0..=32767     => self.mem[addr as usize] = value,
            32768..=32775 => self.reg[self.get_reg(addr)] = value,
            32776..=65535 => panic!("Invalid Address!"),
        }
    }

    fn get_reg(&self, number: u16) -> usize {
        match number {
            32768..=32775 => (number - 32768) as usize,
            _             => panic!("Invalid Register!"),
        }
    }
}

pub fn parse_input(file: &str) -> Result<Vec<u16>, Box<dyn Error>> {
    let mut f   = File::open(file)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(buf
        .chunks(2)
        .map(|chunk| match *chunk {
            [l, h] => ((h as u16) << 8) + (l as u16),
            _      => panic!("Invalid Input!"),
        })
        .collect())
}
