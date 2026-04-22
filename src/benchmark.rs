
pub struct Benchmark {
    pub name: &'static str,
    pub iterations: u64,
    pub total_cycles: u64,
}

impl Benchmark {
    pub fn new(name: &'static str) -> Self {
        Benchmark { name, iterations: 0, total_cycles: 0 }
    }

    pub fn run(&mut self, iterations: u64, f: fn()) {
        self.iterations = iterations;
        let start = read_tsc();
        for _ in 0..iterations {
            f();
        }
        let end = read_tsc();
        self.total_cycles = end.wrapping_sub(start);
    }

    pub fn report(&self) {
        let avg = if self.iterations > 0 {
            self.total_cycles / self.iterations
        } else { 0 };
        crate::println!("[bench] {}: {} iterations, {} total cycles, {} avg cycles/op",
            self.name, self.iterations, self.total_cycles, avg);
    }
}

/// Read CPU timestamp counter - real hardware cycle count
pub fn read_tsc() -> u64 {
    let lo: u32;
    let hi: u32;
    unsafe {
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
        );
    }
    ((hi as u64) << 32) | (lo as u64)
}
