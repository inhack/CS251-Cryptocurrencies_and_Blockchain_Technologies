pub mod q1 {
    use crate::utils;

    use bitcoin::secp256k1::SecretKey;
    use bitcoin::{Address, KeyPair, PublicKey, PrivateKey, Network};
    use bitcoin::blockdata::opcodes;        // ex. opcodes::all::OP_RETURN;
    use bitcoin::blockdata::script::{Script, Builder};
    use bitcoin::blockdata::transaction::{TxIn, TxOut};
    

    /////////////////////////////////////////////////////////////////////////////////////
    /////////////////////////////////     Qustion 1     /////////////////////////////////
    /////////////////////////////////////////////////////////////////////////////////////
    fn qustion_1() {
        let amount_to_send: Option<u64> = None;
        let txid_to_spend: String = String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX");
        let utxo_index: Option<u64> = None;

        // let txout_scriptPubkey = P2PKH_scriptPubKey("faucet");   // config's faucet address
        // let resp = send_from_P2PKH_transaction(amount_to_send, txid_to_spend, utxo_index, txout_scriptPubkey, my_private_key, Network::Testnet);

    }

    #[allow(non_snake_case)]
    fn P2PKH_scriptPubKey(address: &Address) -> Result<Script, Box<dyn std::error::Error>> {
        let pubkey_hash = address.payload.script_pubkey();

        Ok(Builder::new()
            .push_opcode(opcodes::all::OP_DUP)
            .push_opcode(opcodes::all::OP_HASH160)
            .push_slice(&pubkey_hash[..])
            .push_opcode(opcodes::all::OP_EQUALVERIFY)
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script()
        )

    }

    #[allow(non_snake_case)]
    fn P2PKH_scriptSig(txin: &TxIn, txout: &TxOut, txin_scriptPubKey: &Script, sec_key: &SecretKey, pub_key: &PublicKey) -> Result<Script, Box<dyn std::error::Error>>{
        // opcodes vector to bitcoin::blockdata::script::Script type
        let signature = utils::utils::create_OP_CHECKSIG_signature(txin.clone(), txout.clone(), &txin_scriptPubKey, &sec_key);
        
        Ok(Builder::new()
            .push_slice(&signature.serialize_compact())
            .push_key(&pub_key)
            .into_script()
        )
    }

    #[allow(non_snake_case)]
    fn send_from_P2PKH_transaction(amount_to_send: Option<u64>,
                                    txid_to_spend: String,
                                    utxo_index: Option<u64>,
                                    txout_scriptPubkey: Script,
                                    sender_private_key: PrivateKey,
                                    network: Network) -> Result<(), Box<dyn std::error::Error>> {
        
                

        Ok(())
    }
}