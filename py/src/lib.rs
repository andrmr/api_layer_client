use pyo3::{prelude::*, types::PyString};
use api_layer_client::*;

#[pyfunction]
fn get_list<'p>(py: Python<'p>, api_key: Option<&'p PyString>) -> PyResult<&'p PyAny> {
    let env_key = || std::env::var("APIKEY")
        .expect("Provide an APIKEY value or set an APIKEY env var");

    let api_key = match api_key {
        Some(api_key) => match api_key.to_str() {
            Ok(api_key) if !api_key.is_empty() => api_key.to_owned(),
            _ => env_key()
        }
        _ => env_key()
    };

    let client = ApiLayerClient::new(&api_key);

    pyo3_asyncio::tokio::future_into_py(py, async move {
        client.list().await.map_err(|e| PyErr::from(e))
    })
}

#[pymodule]
fn py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_list, m)?)?;
    Ok(())
}