struct CPU {
    current_operation: u16,
    register: [u8; 2],
}
// 硬件起始就是实现逻辑，能用硬件实现的逻辑，用软件是肯定可以的。
pub fn summator() {
    let mut cpu = CPU {
        current_operation: 0,
        register: [0; 2],
    };
    // 8表示使用2个寄存器，0 寄存器0，1寄存器1，4表示假发操作。
    cpu.current_operation = 0x8014;
    cpu.register[0] = 5;
    cpu.register[1] = 10;
    cpu.run();
    assert_eq!(cpu.register[0], 15);
    println!("5 + 10 = {}", cpu.register[0]);
}

// 读取操作码，解码指令，使用match将解码后的指令匹配到已知的操作码上。
impl CPU {
    fn read_code(&self) -> u16 {
        self.current_operation
    }
    fn run(&mut self) {
        // loop {

        let opcode = self.read_code();
        // 解码
        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            _ => todo!("opcode {:04x}", opcode), //更多指令功能
        }
        //        }
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        self.register[x as usize] += self.register[y as usize];
    }
}
