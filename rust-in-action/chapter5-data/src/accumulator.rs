struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

pub fn accumulator() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;
    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x14;
    mem[4] = 0x80;
    mem[5] = 0x14;
    cpu.run();
    assert_eq!(cpu.registers[0], 35);
    println!("5 + 10 + 10 + 10 = {}", cpu.registers[0])
}

impl CPU {
    fn read_code(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;
        // 因为内存里面一个值是8位，所以这里左移，然后合并。
        op_byte1 << 8 | op_byte2
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_code();
            self.position_in_memory += 2; //指向下一条指令
                                          // 解码
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode), //更多指令功能
            }
        }
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        // 这里其实没用到这个进位,因为val就是最后的值。
        self.registers[x as usize] = val;
        if overflow {
            //最后一个寄存器放进位标志。
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
