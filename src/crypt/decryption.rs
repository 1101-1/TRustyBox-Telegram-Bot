use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt},
    Aes256,
};
use aes_gcm::KeyInit;

pub fn decrypt_data(buf: Vec<u8>, aes_key: [u8; 32]) -> Result<Vec<u8>, tokio::io::Error> {
    let cipher = Aes256::new(&GenericArray::from_slice(&aes_key));
    let mut decrypted_data = buf.to_vec().to_owned();

    for block in decrypted_data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(block);
        cipher.decrypt_block(block);
    }

    let pad_byte = *decrypted_data.last().unwrap();
    let pad_len = pad_byte as usize;

    decrypted_data.truncate(decrypted_data.len() - pad_len);

    Ok(decrypted_data)
}
