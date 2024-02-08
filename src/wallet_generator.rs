use std::sync::mpsc::Sender;
// used for generating hashes using the Keccak-256 algorithm
use std::sync::{Arc, Mutex};
use sha3::{Digest, Keccak256}; // used for generating hashes using the Keccak-256 algorithm
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use rand_chacha::{ChaChaRng, ChaCha20Rng}; // used for generating random numbers
use libsecp256k1::{PublicKey, SecretKey}; 

use crate::args::Parameters;


//generate random wallet
fn generate_wallet(rng: &mut ChaCha20Rng) -> (String, String)  {    
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    
    let secret_key = SecretKey::parse(&bytes).unwrap();

    // Generate a public key from the secret key
    let public_key = PublicKey::from_secret_key(&secret_key);

    // Hash the public key to get the Ethereum address 
    let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = hex::encode(&public_key_hash[12..]);
    (address.to_string(), hex::encode(&secret_key.serialize()).to_string())
}


//thread function to generate wallets, check if they comply with params and send it back through channel
pub fn thread_function(params: Parameters, tx: Sender<(String, String)>, flag: Arc<Mutex<bool>>) {
    let mut rng = ChaChaRng::from_entropy();
    loop {
        let wallet = generate_wallet(&mut rng);
        if !check_address(&wallet.0, &params) {
            continue;
        }
        let flag_value = flag.lock().unwrap();
        if *flag_value {
            break;
        }
        tx.send(wallet).unwrap();
    }
}

//check if address complies with params
fn check_address(address: &String, params: &Parameters) -> bool {

    if let Some(prefix) = &params.p {
        if !address.starts_with(prefix) {
            return false;
        }
    }

    if let Some(suffix) = &params.s {
        if !address.ends_with(suffix) {
            return false;
        }
    }

    if let Some(contains) = &params.a {
        if !address.contains(contains) {
            return false;
        }
    }

    if let (Some(contains_number), Some(contains_char)) = (params.cn, &params.c) {
        if !check_address_contains(address, contains_number, &contains_char) {
            return false;
        }
    }
    true
}

//check if the address contains at least n number of char x
fn check_address_contains(address: &String, n: usize, x: &str) -> bool {
    let count = address.matches(x).count();
    count >= n
}
