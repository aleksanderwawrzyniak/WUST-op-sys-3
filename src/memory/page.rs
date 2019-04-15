#[derive(Debug, Copy, Clone)]
pub struct Page {
    value: u64,
    taken_ticks: u64,
    taken: bool,
}

impl Page {
    pub fn new(value: u64) -> Self {
        Page {
            value,
            taken_ticks: 0_u64,
            taken: false,
        }
    }

    pub fn new_taken(value: u64) -> Self {
        Page {
            value,
            taken_ticks: 1_u64,
            taken: true,
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn is_taken(&self) -> bool {
        self.taken
    }

    pub fn ticks(&self) -> u64 {
        self.taken_ticks
    }

    pub fn set_value(&mut self, val: u64) {
        self.value = val;
    }

    pub fn set_taken(&mut self, state: bool) {
        self.taken = state;
    }

    pub fn set_ticks(&mut self, ticks: u64) {
        self.taken_ticks = ticks;
    }
}
