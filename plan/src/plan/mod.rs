pub mod layers;

pub struct Plan {
    ram: u64,
    cpu: usize,
}
impl Plan {
    #[must_use]
    pub const fn new(ram: u64, cpu: usize) -> Self {
        Self { ram, cpu }
    }
    #[must_use]
    pub const fn get_ram(&self) -> u64 {
        self.ram
    }
    #[must_use]
    pub const fn get_cpu(&self) -> usize {
        self.cpu
    }
}
