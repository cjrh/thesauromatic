use pyo3::prelude::*;

/// Return the related words for `word` as a list of strings (empty if unknown).
#[pyfunction]
fn synonyms(word: &str) -> Vec<String> {
    thesauromatic::lookup(word)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn thesauromatic_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(synonyms, m)?)?;
    Ok(())
}
