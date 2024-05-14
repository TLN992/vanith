use std::sync::mpsc::Sender;
// used for generating hashes using the Keccak-256 algorithm
use std::sync::{Arc, Mutex};
use contract_address::{Address, ContractAddress, U256};
use sha3::{Digest, Keccak256}; // used for generating hashes using the Keccak-256 algorithm
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::rand_core::RngCore;
use rand_chacha::{ChaChaRng, ChaCha20Rng}; // used for generating random numbers
use libsecp256k1::{PublicKey, SecretKey}; 

use std::str::FromStr;
use crate::args::Parameters;
use crate::ValidAddress;


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
pub fn thread_function(params: Parameters, tx: Sender<ValidAddress>, flag: Arc<Mutex<bool>>) {
    let mut rng = ChaChaRng::from_entropy();
    loop {
        let wallet = generate_wallet(&mut rng);

        match check_address(wallet.0, wallet.1, &params) {
            Some(wlt) => {
                tx.send(wlt).unwrap();
            },
            _=>{},
        }

        let flag_value = flag.lock().unwrap();
        if *flag_value {
            break;
        }
    }
}

//check if address complies with params
fn check_address(address: String, private_key: String, params: &Parameters) -> Option<ValidAddress> {

    if let Some(prefix) = &params.p {
        if !address.starts_with(prefix) {
            return None;
        }
    }

    if let Some(suffix) = &params.s {
        if !address.ends_with(suffix) {
            return None;
        }
    }

    let mut possible_ct = Vec::<(String, u32)>::new();


    if let Some(contract_prefix) = &params.cp {
        
        let sender = Address::from_str(address.as_str()).unwrap();
        let one = U256::one();
        let mut nonce = U256::from(0);

        for i in 0..params.cn {
            let contract_address: Address = ContractAddress::from_sender_and_nonce(&sender, &nonce).into();

            let ct_str_address = hex::encode(contract_address.0);

            if ct_str_address.starts_with(contract_prefix) {
                possible_ct.push((ct_str_address, i));
                if possible_ct.len() >= 4 {
                    break;
                }
            }
            nonce = nonce + one;
        }

        if possible_ct.len() == 0 {
            return None;
        }

    }


    Some(ValidAddress { address, private_key, possible_ct })

}