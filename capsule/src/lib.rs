use plan::plan::Plan;
use std::io::Error;
use sysinfo::System;

#[must_use]
pub fn get_host_resource_limits() -> (u64, usize) {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_ram_host = sys.total_memory();
    let total_cpu_host = sys.cpus().len();
    let ram_limit_80 = (total_ram_host * 8) / 10;
    let cpu_limit_80 = (total_cpu_host * 8) / 10;
    (ram_limit_80, cpu_limit_80)
}

pub struct Capsule {
    plan: Plan,
    limits: (u64, usize),
}
impl Capsule {
    #[must_use]
    pub fn new(plan: Plan) -> Self {
        let limits = get_host_resource_limits();
        Self { plan, limits }
    }
    ///
    /// Run the capsule
    /// # Errors
    /// if memory or cpu limits are zero
    /// if plan is too big for host
    ///
    pub fn run(&self) -> Result<(), Error> {
        if self.limits.0 == 0 || self.limits.1 == 0 {
            return Err(Error::other(
                "Host resource limits cannot be zero".to_string(),
            ));
        } else if self.limits.0 > self.plan.get_ram() || self.limits.1 > self.plan.get_cpu() {
            return Err(Error::other("Plan is too big for host".to_string()));
        }
        Ok(())
    }
}
