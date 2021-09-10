/// Git Extractor
/// 
/// A little tool to see which information we can extract from a git repo,
/// in particular see what we can achieve with trailers and implement signatures in there.
/// 
/// TODO: Verify signature with listing of public keys

use git2::{Repository, Oid};
use serde::{Serialize, Deserialize};
use serde_json;
use std::env;
use std::fs;
use git_extractor::{parse, Trailer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata<'a> {
  pub sig: &'a str,
  pub msg: &'a str,
  pub trailers: Vec<Trailer<'a>>,
}

fn main() {
  // Get the repo reference
  let repo = match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  };

  let id = env::args().nth(1).unwrap();

  // Extract the gpg signature and signed data from commit
  let oid = Oid::from_str(&id).expect("Error parsing commit id");
  let commit = repo.find_commit(oid).expect("Error finding commit"); 
  let extract = repo.extract_signature(&commit.id(), None).expect("Error extracting sig"); 

  let sig = extract.0.as_str().unwrap();
  let msg = extract.1.as_str().unwrap();

  let trailers = parse(msg, ":").expect("Error parsing trailers");

  let result = Metadata {
    sig,
    msg,
    trailers
  };

  fs::write("output.json",serde_json::to_string(&result).expect("Error parsing json")).expect("Error writing file");
}
