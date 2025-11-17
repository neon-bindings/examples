#[neon::export]
fn get_num_cpus() -> f64 {
    num_cpus::get() as f64
}
