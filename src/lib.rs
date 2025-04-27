mod expressions;
mod pl_legacy_hashing;

use pyo3::types::PyModule;
use pyo3::{pymodule, PyResult, Python};
#[pymodule]
fn _internal(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
