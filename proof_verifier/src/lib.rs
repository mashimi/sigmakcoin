use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod zk_circuit;
use crate::zk_circuit::{generate_gradient_proof, verify_gradient_proof};

#[pyfunction]
fn create_proof(loss_before: f32, loss_after: f32, gradient_norm: f32, steps: usize) -> PyResult<Vec<u8>> {
    Ok(generate_gradient_proof(loss_before, loss_after, gradient_norm, steps))
}

#[pyfunction]
fn verify_proof_bytes(proof: Vec<u8>, threshold: u32) -> PyResult<bool> {
    match verify_gradient_proof(&proof, threshold) {
        Ok(result) => Ok(result),
        Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
    }
}

#[pymodule]
fn sigmak_zk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_proof, m)?)?;
    m.add_function(wrap_pyfunction!(verify_proof_bytes, m)?)?;
    Ok(())
}