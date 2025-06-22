use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use clap::{Parser, Subcommand};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct DidDocument {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    verification_method: Vec<VerificationMethod>,
    authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    assertion_method: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
struct VerificationMethod {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    controller: String,
    #[serde(rename = "publicKeyMultibase")]
    public_key_multibase: String,
}

#[derive(Serialize, Deserialize)]
struct Jwk {
    kty: String,
    crv: String,
    d: String,
    x: String,
}

fn generate_keypair() -> SigningKey {
    let mut csprng = OsRng {};
    SigningKey::generate(&mut csprng)
}

fn public_key_to_did_key(public_key: &VerifyingKey) -> String {
    // Prefix for Ed25519 multicodec: 0xed01
    let mut key_bytes = vec![0xed, 0x01];
    key_bytes.extend_from_slice(public_key.as_bytes());
    // Encode with multibase 'z' (base58-btc)
    let multibase = format!("z{}", bs58::encode(key_bytes).into_string());
    format!("did:key:{}", multibase)
}

fn create_did_document(did: &str, public_key: &VerifyingKey) -> DidDocument {
    let public_key_multibase = format!("z{}", bs58::encode(public_key.as_bytes()).into_string());
    let verification_method = VerificationMethod {
        id: format!("{}#{}", did, public_key_multibase),
        type_: "Ed25519VerificationKey2020".to_string(),
        controller: did.to_string(),
        public_key_multibase,
    };
    DidDocument {
        context: vec![
            "https://www.w3.org/ns/did/v1".to_string(),
            "https://w3id.org/security/suites/ed25519-2020/v1".to_string(),
        ],
        id: did.to_string(),
        verification_method: vec![verification_method.clone()],
        authentication: vec![verification_method.id.clone()],
        assertion_method: vec![verification_method.id],
    }
}

fn create_jwk(keypair: &SigningKey) -> Jwk {
    Jwk {
        kty: "OKP".to_string(),
        crv: "Ed25519".to_string(),
        d: STANDARD_NO_PAD.encode(keypair.as_bytes()),
        x: STANDARD_NO_PAD.encode(keypair.verifying_key().as_bytes()),
    }
}

fn write_json_file<T: Serialize>(filename: &str, data: &T) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[derive(Parser)]
#[command(
    author = "thanhtan541",
    version = "0.1.0",
    about = "RSA encryption/decryption CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate Did documents and its associated keys for multiple users")]
    GenerateUserDids,
    #[command(about = "Generate rooms for multiple users")]
    GenerateRooms,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateUserDids => {
            println!("Generating user DIDs...");

            let users = [
                "alice", "bob", "carol", "daniel", "ethan", "faye", "gavin", "henry", "ian", "jade",
            ];

            let data_dir = "data";
            if Path::new(data_dir).exists() {
                remove_dir_all(data_dir).expect("Failed to clean all files");
            }

            // Create data directory if it doesn't exist
            create_dir_all(data_dir).expect("Failed to create directory");

            for user in users.iter() {
                // Generate keypair
                let keypair = generate_keypair();

                // Create DID
                let did = public_key_to_did_key(&keypair.verifying_key());

                // Create DID document
                let did_doc = create_did_document(&did, &keypair.verifying_key());
                let did_filename = format!("{}/{}-did.json", data_dir, user);
                write_json_file(&did_filename, &did_doc).expect("Failed to write to file");

                // Create JWK
                let jwk = create_jwk(&keypair);
                let jwk_filename = format!("{}/{}-private-key.json", data_dir, user);
                write_json_file(&jwk_filename, &jwk).expect("Failed to write to file");
            }
        }
        Commands::GenerateRooms => {
            println!("Generating rooms...");
        }
    }
}
