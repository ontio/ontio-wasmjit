use ripemd160::Digest;

pub(crate) fn contract_address(code: &[u8]) -> [u8; 20] {
    let sha = hmac_sha256::Hash::hash(code);
    let result = ripemd160::Ripemd160::digest(&sha);
    let mut address = [0; 20];

    address.as_mut().copy_from_slice(&result);
    address
}
