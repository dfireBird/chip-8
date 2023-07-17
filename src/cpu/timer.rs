pub struct Timer {
    value: u8,
}

impl Timer {
    pub fn sub(&mut self) {
        self.value = self.value - 1;
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer { value: 0 }
    }
}
