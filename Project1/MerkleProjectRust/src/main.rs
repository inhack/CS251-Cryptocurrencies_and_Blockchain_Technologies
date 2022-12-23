use MerkleProjectRust::merkle_proof::MerkleProof;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    /*
        prover
    */ 
    let leaves = (0..1000).map(|i| format!("data item {}", i)).collect::<Vec<String>>();
    // println!("{:?}", leaves);
    println!("[+] Generated 1000 leaves for a Merkle tree of height 10");

    let pos: u32 = 95;
    let merkle_proof_for_prover = MerkleProof::new().gen_merkle_proof(leaves[pos as usize].clone(), leaves, pos)?;

    merkle_proof_for_prover.write_proof()?;

    /*
        verifier
    */
    let merkle_proof_for_verifier = MerkleProof::read_proof("proof-for-leaf-95.txt")?;
    let calculated_root = merkle_proof_for_verifier.compute_merkle_root_from_proof()?;

    if merkle_proof_for_verifier.root == calculated_root {
        // println!("[+] verify succeed");
        println!("[+] {} (position: {}) verify succeed !",  merkle_proof_for_verifier.leaf, merkle_proof_for_verifier.pos);
        println!("  - root : {}", base64::encode(merkle_proof_for_verifier.root));
        println!("  - calculated root from proof : {}", base64::encode(calculated_root));
    } else {
        println!("[-] {} (position: {}) verify failed ..", merkle_proof_for_verifier.leaf, merkle_proof_for_verifier.pos);
        println!("  - root : {}", base64::encode(merkle_proof_for_verifier.root));
        println!("  - calculated root from proof : {}", base64::encode(calculated_root));
    }
    

    Ok(())

}
