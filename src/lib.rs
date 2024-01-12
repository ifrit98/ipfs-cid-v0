use pyo3::prelude::*;
use pyo3::types::PyBytes;
use ipfs_unixfs::file::adder::FileAdder;
use multihash::{Code, MultihashDigest};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compute_cid(data: Vec<u8>) -> PyResult<String> {
    let mut adder = ipfs_unixfs::file::adder::FileAdder::default();
    let size_hint = adder.size_hint();

    let mut written: usize = 0;
    while written < data.len() {
        let end = written + (data.len() - written).min(size_hint);
        let slice = &data[written..end];

        let (_blocks, pushed) = adder.push(slice);
        written += pushed;
    }

    let last_blocks = adder.finish();
    let (cid, _block) = last_blocks.last().unwrap();

    Ok(cid.to_string())
}


/// Computes the SHA-256 hash of the data formatted as per UnixFS.
#[pyfunction]
fn compute_hash(py: Python, data: Vec<u8>) -> PyResult<PyObject> {
    let mut adder = FileAdder::default();

    let mut written: usize = 0;
    while written < data.len() {
        let end = written + (data.len() - written).min(adder.size_hint());
        let slice = &data[written..end];
        adder.push(slice);
        written = end;
    }

    let last_blocks = adder.finish();
    let (_cid, block) = last_blocks.last().unwrap();

    // Compute SHA-256 hash
    let hash = Code::Sha2_256.digest(&block);

    // Convert hash to Python bytes
    Ok(PyBytes::new(py, &hash.to_bytes()).into())
}


/// Computes the SHA-256 hash of the data formatted as per UnixFS.
#[pyfunction]
fn compute_hash_hex(data: Vec<u8>) -> PyResult<String> {
    let mut adder = FileAdder::default();

    let mut written: usize = 0;
    while written < data.len() {
        let end = written + (data.len() - written).min(adder.size_hint());
        let slice = &data[written..end];
        adder.push(slice);
        written = end;
    }

    let last_blocks = adder.finish();
    let (_cid, block) = last_blocks.last().unwrap();

    // Compute SHA-256 hash
    let hash = Code::Sha2_256.digest(&block); // Pass a reference to the Vec<u8>
    
    // Convert hash to a hex string
    let hash_hex = hash.to_bytes().iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    Ok(hash_hex)
}


/// A Python module implemented in Rust.
#[pymodule]
fn ipfs_cid_v0(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_cid, m)?)?;
    m.add_function(wrap_pyfunction!(compute_hash, m)?)?;
    m.add_function(wrap_pyfunction!(compute_hash_hex, m)?)?;
    Ok(())
}
