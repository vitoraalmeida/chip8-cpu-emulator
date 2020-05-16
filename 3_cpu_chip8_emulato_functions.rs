/*
Para simular a chamada de uma função de multiplicação por dois, a função de
adição vai ser chamada duas vezes para o mesmo numero.
*/

struct CPU {
    registers: [u8; 16],
    program_counter: usize, //rastreia onde estamos no programa
    memory: [u8; 4096], //vai ser usada para armazenar bytes contendo partes
                        //de opcodes. Cada espaço comporta 1 byte.
    stack: [u16; 16],   //armazena endereços de memoria para funções
    stack_pointer: usize, //rastreia qual função está sendo executada
}


impl CPU {
    fn run(&mut self) {
        loop {
            let op_byte1 = self.memory[self.program_counter] as u16; //Ex.: 0x80
            let op_byte2 = self.memory[self.program_counter + 1] as u16;//Ex.: 0x14
            let opcode = op_byte1 << 8 | op_byte2; //Ex.: 0x8014
            
            /*
            Ex.: 0x8014

            1111 0000 0001 0100
            0000 1111 0000 0000
            -------------------
            0000 0000 0000 0000  >> 8 == 0000 0000 0000 0000
            */
            let x =        ((opcode & 0x0F00) >>  8) as u8;

            /*
            1111 0000 0001 0100
            0000 0000 1111 0000
            -------------------
            0000 0000 0001 0000  >> 4 == 0000 0000 0000 0001
            */
            let y =        ((opcode & 0x00F0) >>  4) as u8;

            /*
            1111 0000 0001 0100
            0000 0000 0000 1111
            -------------------
            0000 0000 0000 0100  == 0000 0000 0000 0100
            */
            let op_minor = (opcode & 0x000F) as u8;

            /*
            1111 0000 0001 0100
            0000 1111 1111 1111
            -------------------
            0000 0000 0001 0100  == 0000 0000 0001 0100
            */
            let addr =     opcode & 0x0FFF;

            self.program_counter += 2; //cada opcode ocupa 2 espaços em memoria

            match opcode {
                0x0000 => { return; }, // HALT
                0x00EE => { self.ret(); }, //RETURN mod o pc para a memoria da call anterior
                0x2000...0x2FFF => { self.call(addr); }, //CALL modifica o pc para o endereço
                0x8000...0x8FFF => {                     //onde está a função
                    match op_minor { // se é 8XXX, é logica ou aritmética
                        4 => { self.add_xy(x, y); }
                        _ => { unimplemented!("opcode: {:04x}", opcode); },
                    }
                },
                _ => unimplemented!("opcode {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {   // funcao-------||-|------- endereço da função
        //para o caso do opcode ser chamada de função 2100
        let sp = self.stack_pointer; //inicialmente 0
        let stack = &mut self.stack; // pega informação da  pilha de chamadas
                                     //que contem os endereços das funções chamadas
        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.program_counter as u16; //stack[0] = 2
        self.stack_pointer += 1; //coloca na pilha
        self.program_counter = addr as usize; //pc = 0x100 endereço da função
                                              //no proximo ciclo, o opcode será
                                              //o contido no endereço 100
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1; //retira da pilha
        self.program_counter = self.stack[self.stack_pointer] as usize;
        //pc volta para 2, que seria o valor do proximo opcode antes da
        //função
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
        stack: [0; 16],
        stack_pointer: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    //inicio do programa a ser executado
    cpu.memory[0x000] = 0x21; cpu.memory[0x001] = 0x00; //primeira chamada da função
    cpu.memory[0x002] = 0x21; cpu.memory[0x003] = 0x00; //segunda
    //depois daqui o pc aponta para um espaço vazio de memoria, sinal para acabar.

    cpu.memory[0x100] = 0x80; cpu.memory[0x101] = 0x14; //quando chega aqui, duas
    cpu.memory[0x102] = 0x80; cpu.memory[0x103] = 0x14; //adições acontencem e
    cpu.memory[0x104] = 0x00; cpu.memory[0x105] = 0xEE; //o retorno é chamado

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
