pub struct Timer(u8);

impl Timer {
    pub fn sub(&mut self) {
        if self.0 > 0 {
            self.0 = self.0 - 1;
        }
    }

    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer(0)
    }
}
