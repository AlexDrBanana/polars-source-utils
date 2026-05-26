use pyo3::prelude::*;
use pyo3::types::PyDict;

mod plan_paths;

#[pymodule]
fn _internal(_py: Python<'_>, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    _m.add_function(wrap_pyfunction!(list_source_paths_py, _m)?)?;
    _m.add_function(wrap_pyfunction!(replace_source_paths_py, _m)?)?;
    Ok(())
}

#[pyfunction(name = "list_source_paths")]
fn list_source_paths_py(path: &str) -> PyResult<Vec<String>> {
    plan_paths::list_source_paths(std::path::Path::new(path))
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

#[pyfunction(name = "replace_source_paths")]
fn replace_source_paths_py(path: &str, mapper: &Bound<'_, PyDict>) -> PyResult<usize> {
    let mut map = std::collections::HashMap::with_capacity(mapper.len());
    for (k, v) in mapper.iter() {
        let key: String = k.extract()?;
        let value: String = v.extract()?;
        map.insert(key, value);
    }
    plan_paths::replace_source_paths(std::path::Path::new(path), &map)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}