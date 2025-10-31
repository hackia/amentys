use sysinfo::System;

pub mod layers;

pub struct Plan {
    ram: u64,
    cpu: usize,
    layers: layers::Layers,
    os_ram: u64,
    os_cpu: usize,
}
impl Plan {
    #[must_use]
    /// # Panics
    /// if ram is superior to host ram and cpu is superior to host cpu
    pub fn new(ram: u64, cpu: usize) -> Self {
        let mut os = System::new_all();
        os.refresh_all();
        let os_ram = os.available_memory();
        let os_cpu = os.cpus().len();
        assert!(
            !(ram > os_ram || cpu > os_cpu),
            "Insufficient system resources for plan creation"
        );
        Self {
            ram,
            cpu,
            layers: layers::Layers::new(),
            os_ram,
            os_cpu,
        }
    }
    #[must_use]
    pub const fn get_os_ram(&self) -> u64 {
        self.os_ram
    }
    #[must_use]
    pub const fn get_os_cpu(&self) -> usize {
        self.os_cpu
    }

    #[must_use]
    pub const fn get_ram(&self) -> u64 {
        self.ram
    }
    #[must_use]
    pub const fn get_cpu(&self) -> usize {
        self.cpu
    }
    pub fn add_layer(&mut self, layer: layers::Layer) -> &mut Self {
        self.layers.add(layer);
        self
    }
    #[must_use]
    pub const fn get_layers(&self) -> &layers::Layers {
        &self.layers
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plan_creation() {
        let one = layers::Layer::new("one");
        let mut plan = Plan::new(2048, 2);
        plan.add_layer(one);
        assert_eq!(plan.get_ram(), 2048);
        assert_eq!(plan.get_cpu(), 2);
        assert_eq!(plan.get_layers().len(), 2);
        assert_eq!(
            plan.get_layers().first().unwrap().get_name(),
            layers::ROOT_LAYER_NAME
        );
        assert_eq!(plan.get_layers().last().unwrap().get_name(), "one");
    }
    #[test]
    #[should_panic]
    fn test_plan_creation_panic() {
        let plan = Plan::new(90909090, 5000);
        assert_eq!(plan.get_ram(), 90909090);
        assert_eq!(plan.get_cpu(), 5000);
    }
}
