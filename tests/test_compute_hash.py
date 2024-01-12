import pytest
from py_ipfs_cid import compute_hash, compute_hash_hex
from pathlib import Path

# Tests for compute_hash()
@pytest.mark.parametrize(
    "data, expected_hash",
    [
        (b"1234", b'\x12 K\x1b\x80\x9cr\xce\x0e\xf8\xd8\x15\xb9"\xeef\x04/\xc4\xaf\xe5\xc9G\xad\xd2\xa5,\xbcR\xd8\xb0\x19_\xa3'),
        (b"", b'\x12 \xbf\xcc\xdax{\xab\xa3+Y\xc7\x84P\xac= \xb636\x0bC\x99,w(\x9f\x9e\xd4m\x845a\xe6')
    ]
)
def test_compute_hash(data, expected_hash):
    computed_hash = compute_hash(data)
    assert computed_hash == expected_hash


# Tests for compute_hash_hex()
@pytest.mark.parametrize(
    "data, expected_hash_hex",
    [
        (b"1234", "12204b1b809c72ce0ef8d815b922ee66042fc4afe5c947add2a52cbc52d8b0195fa3"),
        (b"", "1220bfccda787baba32b59c78450ac3d20b633360b43992c77289f9ed46d843561e6")
    ]
)
def test_compute_hash_hex(data, expected_hash_hex):
    computed_hash_hex = compute_hash_hex(data)
    assert computed_hash_hex == expected_hash_hex
