pub mod config {
    use bitcoin::Address;
    use bitcoin::secp256k1::{SecretKey, PublicKey};

    #[allow(non_snake_case)]
    #[derive(Debug)]
    pub struct Config {
        network_name: String,
        faucet_address: Address,
        network_type: String,

        // for Question 1-3
        my_private_key: SecretKey,    // 32
        my_public_key: PublicKey, // 33 , from_secret_key()
        my_address: Address,

        // for Question 4
        alice_secret_key_BTC: SecretKey,
        alice_public_key_BTC: PublicKey,
        alice_address_BTC: Address,

        bob_secret_key_BTC: SecretKey,
        bob_public_key_BTC: PublicKey,
        bob_address_BTC: Address,

        alice_secret_key_BCY: SecretKey,
        alice_public_key_BCY: PublicKey,
        alice_address_BCY: Address,

        bob_secret_key_BCY: SecretKey,
        bob_public_key_BCY: PublicKey,
        bob_address_BCY: Address
    }

    pub fn new() {
        let faucet_address: Address = "mohjSavDdQYHRYXcS3uS6ttaHP8amyvX78".parse().unwrap();
    }
}

pub mod keygen {
    use rand::Rng;
    use bitcoin::{Address, KeyPair, PublicKey, PrivateKey};
    use bitcoin::network::constants::Network;
    use bitcoin::secp256k1::{Secp256k1, SecretKey};

    pub fn gen_secret_key() -> Result<(SecretKey, PrivateKey, PublicKey, Address), Box<dyn std::error::Error>> {
        let secret_bytes = rand::thread_rng().gen::<[u8; 32]>();

        let secp = Secp256k1::new();

        // let key_pair = KeyPair::new(&secp, &mut rand::thread_rng());     // ::new() is not working..
        let key_pair = KeyPair::from_seckey_slice(&secp, &secret_bytes)?;

        let priv_key = PrivateKey::new(key_pair.secret_key(), Network::Testnet);

        let pub_key = PublicKey::from_private_key(&secp, &priv_key);

        let address = Address::p2pkh(&pub_key, Network::Testnet);
        
        Ok((key_pair.secret_key(), priv_key, pub_key, address))
    }
}

pub mod split_test_coins {

}

pub mod utils {
    
    use bitcoin::hashes::sha256;
    use bitcoin::secp256k1::{SecretKey, Message, Secp256k1};
    use bitcoin::secp256k1::ecdsa::Signature;

    use bitcoin::blockdata::script::Script;
    use bitcoin::blockdata::locktime::PackedLockTime;
    use bitcoin::blockdata::transaction::{TxIn, TxOut, Transaction, SigHashType};

    pub fn create_OP_CHECKSIG_signature(tx_in: TxIn, tx_out: TxOut, tx_in_scriptPubkey: &Script, sec_key: &SecretKey) -> Signature {
        
        // Unsigned Tx
        let tx = Transaction {
			version: 1,
			lock_time: PackedLockTime(0),
			input: vec![tx_in],
			output: vec![tx_out],
		};

        let sig_hash = tx.signature_hash(0, &tx_in_scriptPubkey, SigHashType::All as u32);    // SIGHASH_ALL

        let secp = Secp256k1::new();
        let msg = Message::from_hashed_data::<sha256::Hash>(&sig_hash.to_vec());
        let checksig = secp.sign_ecdsa(&msg, &sec_key);

        checksig
    }
}