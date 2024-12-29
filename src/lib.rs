use anyhow::Result;
use pyo3::prelude::*;

mod fetcher;

#[pymodule]
fn air_browser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(browse, m)?)?;
    Ok(())
}
