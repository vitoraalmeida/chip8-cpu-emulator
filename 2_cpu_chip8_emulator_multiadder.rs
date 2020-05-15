const ARITHMETIC_AND_LOGIC: u8 = 0x8;
const HALT: u8 = 0x0;
const ADD_XY: u8 = 0x4;

struct CPU {
    registers: [u8; 16],
    program_counter: usize, //rastreia onde estamos no programa
    memory: [u8; 4096], //vai ser usada para armazenar bytes contendo partes 
}                       //de opcodes. Cada espaço comporta 1 byte.

impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.program_counter] as u16;
            let op_byte2 = self.memory[self.program_counter + 1] as u16;
            let raw_op = op_byte1 << 8 | op_byte2;

            let op_major = ((raw_op & 0xF000) >> 12) as u8;
            let x =        ((raw_op & 0x0f00) >>  8) as u8;
            let y =        ((raw_op & 0x00F0) >>  4) as u8;
            let op_minor = (raw_op & 0x000F) as u8;

            self.program_counter += 2;

            match (op_major, op_minor) {
                (HALT, HALT) => { return; },
                (ARITHMETIC_AND_LOGIC, ADD_XY) => self.add_xy(x,y),
                _ => unimplemented!("opcode {:04x}", raw_op),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}


fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        program_counter: 0,
        memory: [0; 4096],
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    cpu.memory[0] = 0x80; cpu.memory[1] = 0x14;
    cpu.memory[2] = 0x80; cpu.memory[3] = 0x24;
    cpu.memory[4] = 0x80; cpu.memory[5] = 0x34;

    cpu.run();

    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0]);
}
