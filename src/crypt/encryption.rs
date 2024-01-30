use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt},
    Aes256,
};
use aes_gcm::KeyInit;

pub fn encrypt_data(data: &[u8], aes_key: [u8; 32]) -> Result<Vec<u8>, tokio::io::Error> {
    let cipher = Aes256::new(&GenericArray::from_slice(&aes_key));
    let mut padded_data = data.to_vec();
    let pad_len = 16 - (data.len() % 16);
    let pad_byte = pad_len as u8;

    padded_data.resize(data.len() + pad_len, pad_byte);

    for block in padded_data.chunks_exact_mut(16) {
        let block = GenericArray::from_mut_slice(block);
        cipher.encrypt_block(block);
    }

    Ok(padded_data)
}
