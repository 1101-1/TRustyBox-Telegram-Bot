use base64::{engine::general_purpose, Engine};

pub fn convert_base64_to_aes(aes_key: String) -> Result<[u8; 32], tokio::io::Error> {
    let key_vec = match general_purpose::URL_SAFE_NO_PAD.decode(aes_key) {
        Ok(key) => key,
        Err(_err) => {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::Other,
                "Can not convert to base64",
            ))
        }
    };

    let key_array: [u8; 32] = match key_vec.try_into() {
        Ok(key) => key,
        Err(_err) => {
            return Err(tokio::io::Error::new(
                tokio::io::ErrorKind::Other,
                "Can not convert aes_key to bytes",
            ))
        }
    };
    Ok(key_array)
}

pub fn convert_aes_to_base64(aes_bytes: [u8; 32]) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(aes_bytes)
}
