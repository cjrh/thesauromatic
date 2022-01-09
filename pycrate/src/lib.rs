use pyo3::prelude::*;

#[pyfunction]
fn synonyms(word: &str) -> PyResult<Vec<String>> {
    let out = thesauromatic::lookup(word);
    Ok(out)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn thesauromatic_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(synonyms, m)?)?;

    Ok(())
}
