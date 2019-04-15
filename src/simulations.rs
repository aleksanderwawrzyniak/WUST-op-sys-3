use super::memory::Memory;

pub struct Simulator {
    number_of_simulations: u64
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            0_u64
        }
    }

    pub fn fifo() {
        number_of_simulations += 1;
        let no_pages = 4;
        let mut mem = Memory::new(no_pages);
        mem.simulate_fifo();
    }
}
