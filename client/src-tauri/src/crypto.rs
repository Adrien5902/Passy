use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};

pub fn cipher(key: &[u8; 32], data: &[u8]) -> Result<(Vec<u8>, [u8; 12]), aes_gcm::Error> {
    let aes_key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    cipher
        .encrypt(&nonce, data)
        .and_then(|res| Ok((res, nonce.into())))
}

pub fn decipher(
    key: &[u8; 32],
    nonce: [u8; 12],
    encrypted_data: &[u8],
) -> Result<Vec<u8>, aes_gcm::Error> {
    let aes_key: &Key<Aes256Gcm> = key.into();

    let cipher = Aes256Gcm::new(aes_key);
    cipher.decrypt(&nonce.into(), encrypted_data)
}
