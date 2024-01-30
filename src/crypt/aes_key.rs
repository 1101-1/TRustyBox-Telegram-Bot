use rand::thread_rng;
use rand_core::RngCore;

pub fn set_aes_key() -> [u8; 32] {
    let mut gen_aes_key = [0u8; 32];
    thread_rng().fill_bytes(&mut gen_aes_key);
    gen_aes_key
}
