struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
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
            let nnn = opcode & 0x0FFF;
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode), //更多指令功能
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        if sp > stack.len() {
            panic!("Stack overflow");
        }
        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }
        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        // 这里其实没用到这个进位,因为val就是最后的值。
        self.registers[x as usize] = val;
        if overflow_detected {
            //最后一个寄存器放进位标志。
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

pub fn call_func() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        position_in_memory: 0,
        stack: [0; 16],
        stack_pointer: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    let mem = &mut cpu.memory;
    // 含义：
    // 首先读取2100操作吗。发现调用地址为100的的内容。
    // 读取100的地址，发现是8014，然后做了操作。内存指针默认都会自增，所以会继续调用102，104
    // 104地址操作码是0x00EE，这是返回标志，会回到调用100时保存的内存指针地址处。也就是002处。
    // 然后002地址还是重复8014。重复后到了004。结束。
    mem[0x000] = 0x21; // 0x2100 在0x100处调用函数。
    mem[0x001] = 0x00;

    mem[0x002] = 0x21; // 0x2100
    mem[0x003] = 0x00;

    mem[0x004] = 0x00; // 0x0000
    mem[0x005] = 0x00;

    mem[0x100] = 0x80; // 0x8014
    mem[0x101] = 0x14;

    mem[0x102] = 0x80; // 0x8014
    mem[0x103] = 0x14;

    mem[0x104] = 0x00; // 0x00EE
    mem[0x105] = 0xEE;

    cpu.run();
    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0])
}

// 理解： 操作码都在内存里面，然后根据启动的默认的内存地址来操作。
// 每次操作完后默认内存地址会增加2，栈用来保存要返回的地址。
