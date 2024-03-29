# Python IPFS CID

Computes IPFS CIDv0s exposed as a Python module.

This library computes the IPFS DAG of the input data and then returns the CID of the root node.
This returns the same CID as would be computed by `ipfs add <file>`.

This library also computes the hash of the data after it has been converted to the DAG-PB format
and processed, primarily for debugging and testing purposes.

## Examples

```python
from ipfs_cid_v0 import compute_cid
cid: str = compute_cid(b"1234")  # Returns QmTPqcLhVnCtjoYuCZwPzfXcFrUviiPComTepHfEEaGf7g
```

```python
from ipfs_cid_v0 import compute_hash
cid_hash: bytes = compute_hash(b"1234") # Returns b'\x12 K\x1b\x80\x9cr\xce\x0e\xf8\xd8\x15\xb9"\xeef\x04/\xc4\xaf\xe5\xc9G\xad\xd2\xa5,\xbcR\xd8\xb0\x19_\xa3'
```

```python
from ipfs_cid_v0 import compute_hash_hex
cid_hash_hex: str = compute_hash_hex(b"1234") # Returns '12204b1b809c72ce0ef8d815b922ee66042fc4afe5c947add2a52cbc52d8b0195fa3'
```

## Build
You must have rust/cargo installed. Then run:
```bash
pip install maturin
maturin develop --release 
```
Install in your python environment if doesn't automatically get picked up:
```bash
python -m pip install ./target/wheels/ipfs_cid_v0-0.0.1-cp310-cp310-manylinux_2_34_x86_64.whl
```

## Current limitations

* Only supports CID v0
* Only supports files, not directories
* Only supports the default IPFS block size (256 KiB)
