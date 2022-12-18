use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use rand_chacha::{ChaChaRng, ChaCha20Rng}; // used for generating random numbers
use libsecp256k1::{PublicKey, SecretKey}; // used for generating secret and public keys
use sha3::{Digest, Keccak256}; // used for generating hashes using the Keccak-256 algorithm
use std::{
    collections::HashMap, // used for storing command line arguments
    sync::{Arc, Mutex},
};
use std::sync::mpsc::Sender; // used for sending messages between threads


//generate a random local wallet
fn generate_wallet(rng: &mut ChaCha20Rng) -> (String, String)  {    
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);
    // Generate a secret key
    let secret_key = SecretKey::parse(&bytes).unwrap();
    // Generate a public key from the secret key
    let public_key = PublicKey::from_secret_key(&secret_key);
    // Hash the public key to get the Ethereum address
    let public_key_hash = Keccak256::digest(&public_key.serialize()[1..]);
    let address = hex::encode(&public_key_hash[12..]);
    (address.to_string(), hex::encode(&secret_key.serialize()).to_string())
}

//thread function to generate wallets, check if they comply with params and send it back through channel
pub fn thread_function(params: HashMap<String, String>, tx: Sender<(String, String)>, flag: Arc<Mutex<bool>>) {
    let mut rng = ChaChaRng::from_entropy();
    loop {
        let wallet = generate_wallet(&mut rng);
        let address = &wallet.0;
        if !check_address(address, &params) {
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
fn check_address(address: &String, params: &HashMap<String, String>) -> bool {
    if params.contains_key("p") {
        if !address.starts_with(params.get("p").unwrap()) {
            return false;
        }
    }
    if params.contains_key("s") {
        if !address.ends_with(params.get("s").unwrap()) {
            return false;
        }
    }
    if params.contains_key("a") {
        if !address.contains(params.get("a").unwrap()) {
            return false;
        }
    }
    true
}