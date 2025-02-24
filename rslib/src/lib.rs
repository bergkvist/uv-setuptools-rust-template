#[pyo3::pymodule]
mod rslib {
    #[pyo3::pyfunction]
    pub fn add(left: u64, right: u64) -> u64 {
        left + right
    }
}
