use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, Default)]
struct Regfile {
    a: u64,
    b: u64,
    c: u64,
    o: u8,
}

impl Regfile {
    fn with_a(a: u64) -> Self {
        Regfile { a, b: 0, c: 0, o: 0 }
    }
}

#[derive(Clone, Copy)]
struct Cpu<'a, 'memory> {
    ip: usize,
    regs: Regfile,
    mem: Option<&'a Memory<'memory>>,
}

struct Memory<'a>(&'a [u8]);

impl<'a, 'memory> Cpu<'a, 'memory> {
    fn new() -> Self {
        Self { ip: 0, regs: Regfile::default(), mem: None }
    }

    fn with_regs(self, regs: Regfile) -> Self {
        Self { ip: self.ip, regs, mem: self.mem }
    }

    fn with_mem(self, mem: &'a Memory<'memory>) -> Self {
        Self { ip: self.ip, regs: self.regs, mem: Some(mem) }
    }

    fn literal(&self) -> u64 {
        self.mem.unwrap().0[self.ip + 1] as u64
    }

    fn combo(&self) -> u64 {
        let operand = self.mem.unwrap().0[self.ip + 1];

        match operand {
            0..=3 => operand as u64,
            4 => self.regs.a,
            5 => self.regs.b,
            6 => self.regs.c,
            _ => panic!("meh")
        }
    }

    fn step(&mut self) {
        match self.mem.unwrap().0[self.ip] {
            0b000 => self.adv(),
            0b001 => self.bxl(),
            0b010 => self.bst(),
            0b011 => self.jnz(),
            0b100 => self.bxc(),
            0b101 => self.out(),
            0b110 => self.bdv(),
            0b111 => self.cdv(),
            _ => panic!("meh"),
        }

        // XXX: We are intentionally ignoring ip update by `jnz` based on our
        // assumption that `jnz` only appears in the end of the loop.
        if self.mem.unwrap().0[self.ip] != 0b011 {
            self.ip = self.ip + 2;
        }
    }

    fn adv(&mut self) {
        self.regs.a = self.regs.a / (1 << self.combo());
    }

    fn bxl(&mut self) {
        self.regs.b ^= self.literal();
    }

    fn bst(&mut self) {
        self.regs.b = self.combo() & 0b111;
    }

    fn jnz(&mut self) {
        if self.regs.a != 0 {
            self.ip = self.literal() as usize;
        }
    }

    fn bxc(&mut self) {
        self.regs.b ^= self.regs.c;
    }

    fn out(&mut self) {
        self.regs.o = (self.combo() & 0b111) as u8;
    }

    fn bdv(&mut self) {
        self.regs.b = self.regs.a / (1 << self.combo());
    }

    fn cdv(&mut self) {
        self.regs.c = self.regs.a / (1 << self.combo());
    }

    // Assumption: A well-formed program has only a single jnz insn in the end,
    // and outputs a single value per each run, which will be passed via the `o`
    // register in the `Regfile`
    fn run_single_loop(&mut self) -> Regfile {
        loop {
            self.step();

            if self.mem.unwrap().0[self.ip] == 0b011 {
                self.ip = 0;
                break self.regs.clone();
            }
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut outputs = Vec::<u8>::new();

        loop {
            let regs = self.run_single_loop();

            outputs.push(regs.o);

            if regs.a == 0 {
                break;
            }
        }

        outputs
    }
}

fn extend_a<'a, 'memory>(
    cpu: &Cpu<'a, 'memory>,
    a: u64,
    goal: &[u8],
) -> Vec<u64> {
    let mut extneded_as = Vec::<u64>::new();

    for a_lower_bits in 0..8u64 {
        let extended_a = (a << 3) | a_lower_bits;

        let mut cpu1 = cpu.clone()
            .with_regs(Regfile::with_a(extended_a));

        if &cpu1.run() == goal {
            extneded_as.push(extended_a);
        }
    }

    extneded_as
}

fn main() {
    let mem = vec![2,4,1,6,7,5,4,4,1,7,0,3,5,5,3,0];

    let mem = Memory(&mem);

    let mut cpu = Cpu::new()
        .with_regs(Regfile::with_a(37293246))
        .with_mem(&mem);

    let outputs = cpu.run();

    println!("{}", outputs.into_iter().map(|o| o.to_string()).join(","));

    // part 2
    let cpu = Cpu::new().with_mem(&mem);

    let mut q = VecDeque::<(u64, usize)>::new();

    q.push_front((0, 0));

    let mut res = Vec::<u64>::new();

    while !q.is_empty() {
        let (a, num_matches) = q.pop_back().unwrap();

        if num_matches >= mem.0.len() {
            res.push(a);
            continue;
        }

        let extended_as = extend_a(
            &cpu,
            a,
            &mem.0[(mem.0.len() - num_matches - 1)..mem.0.len()]
        );

        for extended_a in extended_as {
            q.push_front((extended_a, num_matches + 1));
        }
    }

    println!("{}", res.iter().min().unwrap());
}
