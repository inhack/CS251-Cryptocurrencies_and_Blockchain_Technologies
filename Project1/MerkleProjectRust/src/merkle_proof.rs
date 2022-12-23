use sha2::{Sha256, Digest};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};

use base64; //{encode, decode};
use regex::Regex;

#[derive(Debug)]
pub enum MerkleProofError {
  TooManyLeaves,
}
impl std::error::Error for MerkleProofError {}
impl fmt::Display for MerkleProofError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        MerkleProofError::TooManyLeaves => write!(f, "Too many leaves..!"),
      }
    }
  }

#[derive(Debug, Default)]
pub struct MerkleProof {
    pub leaf: String,
    pub pos: u32,
    pub path: Vec<Vec<u8>>,
    pub root: Vec<u8>
}

impl MerkleProof {
    pub fn new() -> Self {
        Self {
            leaf: Default::default(),
            pos: Default::default(),
            path: Default::default(),
            root: Default::default()
        }
    }

    pub fn hash_leaf(leaf: &String) -> Vec<u8> {
        let mut sha256_hasher = Sha256::new();

        sha256_hasher.update(b"leaf:");
        sha256_hasher.update(leaf.as_bytes());

        sha256_hasher.finalize().to_vec()
    }

    pub fn hash_internal_node(left: &Vec<u8>, right: &Vec<u8>) -> Vec<u8> {
        let mut sha256_hasher = Sha256::new();

        sha256_hasher.update(b"node:");
        sha256_hasher.update(left);
        sha256_hasher.update(right);

        sha256_hasher.finalize().to_vec()
    }

    // for prover
    // return (path, root)
    pub fn gen_merkle_proof(mut self, leaf: String, leaves: Vec<String>, pos: u32) -> Result<Self, Box<dyn std::error::Error>> {
        self.leaf =leaf;
        self.pos = pos;

        let height = (leaves.len() as f32).log2().ceil() as u32;

        if height > 19 {
            return Err(Box::new(MerkleProofError::TooManyLeaves));
        }

        let mut state = leaves.iter().map(|leaf| MerkleProof::hash_leaf(leaf)).collect::<Vec<Vec<u8>>>();

        let padding_len = (2_i32.pow(height) as usize) - leaves.len();

        (0..padding_len).for_each(|_| { state.push(vec![0x0]); });

        let mut path: Vec<Vec<u8>> = Vec::new();

        let mut temp_pos: usize = self.pos as usize;

        for level in 0..height {
            if temp_pos % 2 == 0 {
                path.push(state[temp_pos+1].clone());
            } else {
                path.push(state[temp_pos-1].clone());
            }

            // next level
            let border = (2_i32.pow(height-level)) as usize;
            state = state[0..border].chunks_exact(2).map(|chk| {
                MerkleProof::hash_internal_node(&chk[0], &chk[1])
            }).collect::<Vec<Vec<u8>>>();
            
            temp_pos = temp_pos / 2;
        }

        self.path = path;
        self.root = state[0].clone();

        Ok(self)
    }

    pub fn write_proof(&self) -> Result<(), Box<dyn std::error::Error>> {
        // File Creation
        let file_name = format!("proof-for-leaf-{}.txt", self.pos);
        let mut file = File::create(file_name)?;

        file.write_all(format!("leaf position: {}\n", self.pos).as_bytes())?;
        file.write_all(format!("leaf value : '{}'\n", self.leaf).as_bytes())?;
        file.write_all(format!("root value : '{}'\n", base64::encode(&self.root)).as_bytes())?;
        for p in &self.path {
            file.write_all(format!("  {}\n", base64::encode(p)).as_bytes())?;
        }
        
        Ok(())
    }

    // for verifier
    pub fn read_proof(proof_filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // File open & read
        let file = File::open(proof_filepath)?;
        let reader = BufReader::new(file);

        let mut merkle_proof = MerkleProof::new();

        // parse proof file
        for (i,line) in reader.lines().enumerate() {
            let l = line.unwrap().clone();
            match i {
                0 => {  // pos
                    let re = Regex::new(r"leaf position: (\d*)").unwrap();
                    let pos = re.captures(&l).unwrap();
                    merkle_proof.pos = pos.get(1).unwrap().as_str().parse::<u32>()?;
                },
                1 => {  // leaf
                    let re = Regex::new(r"'(.*)'").unwrap();
                    let leaf = re.captures(&l).unwrap();
                    merkle_proof.leaf = leaf.get(1).unwrap().as_str().to_string();
                },
                2 => {  // root
                    let re = Regex::new(r"'(.*)'").unwrap();
                    let root = re.captures(&l).unwrap();
                    merkle_proof.root = base64::decode(root.get(1).unwrap().as_str().as_bytes()).unwrap();
                },
                _ => {  // path
                    let ll = l.replace(" ", "");
                    merkle_proof.path.push(base64::decode(ll.as_bytes()).unwrap());
                }
            }
        }

        // println!("{:#?}", merkle_proof);
        
        Ok(merkle_proof)
    }

    pub fn compute_merkle_root_from_proof(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut pos = self.pos;
        let mut current_root: Vec<u8> = MerkleProof::hash_leaf(&self.leaf);
        
        for p in &self.path {
            if pos % 2 == 0 {
                current_root = MerkleProof::hash_internal_node(&current_root, &p);
            } else {
                current_root = MerkleProof::hash_internal_node(&p, &current_root);
            }
            pos = pos / 2;
        }

        // println!("Calculated Root : {}", base64::encode(current_root));
        Ok(current_root)
    }
    
}
