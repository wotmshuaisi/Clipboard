use openssl::symm::{decrypt, encrypt, Cipher};

pub fn to_aes(key: &[u8], content: &[u8]) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    let iv = nanoid::generate(16);

    match encrypt(Cipher::aes_256_cbc(), key, Some(iv.as_bytes()), content) {
        Ok(val) => Ok((iv, val)),
        Err(err) => Err(Box::new(err)),
    }
}

pub fn from_aes(key: &[u8], iv: &[u8], data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    match decrypt(Cipher::aes_256_cbc(), key, Some(iv), data) {
        Ok(val) => match String::from_utf8(val) {
            Ok(v) => Ok(v),
            Err(err) => Err(Box::new(err)),
        },
        Err(err) => Err(Box::new(err)),
    }
}

/* Test code */

#[test]
fn encrypt_and_decrypt() {
    use openssl::hash::{hash, MessageDigest};
    let key = hash(MessageDigest::sha3_256(), b"teqweqwrfewgkerest").unwrap();
    let data = "test data wo cao";
    let (iv, content) = to_aes(&key, data.as_bytes()).unwrap();

    let data_decrypted = from_aes(&key, iv.as_bytes(), &content);

    assert_eq!(data, data_decrypted.unwrap().as_str());
}
