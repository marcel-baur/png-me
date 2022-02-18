use crate::args::{EncodingArgs};
use crate::png::Png;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use std::fs;
use std::str::FromStr;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use anyhow::{bail, Result};


pub fn encode(args: &EncodingArgs) -> Result<()>  {
    println!("{:?}", args);
    match fs::read(args.path.clone()) {
        Ok(file) => {
            let mut png = Png::try_from(&file[..]).expect("Could not parse PNG from file.");
            let chunk_type = ChunkType::from_str(&args.chunk_type).expect("Creating chunk failed!");
            let data: Vec<u8> = match &args.key {
                Some(key) => {
                    encrypt(&key, &args.message).as_bytes().to_vec()
                }
                None => {
                    args.message.clone().as_bytes().to_vec()
                }
            };

            if !chunk_type.is_valid() {

                bail!("Invalid chunk type!");
            }
            let chunk = Chunk::new(chunk_type, data);
            png.append_chunk(chunk);
            match &args.output {
                Some(output) => fs::write(output.clone(), png.as_bytes())?,
                None => println!("{png}"),
            };
            Ok(())

        }
        Err(_) => {
            bail!("Could not read file!")
        }
    }
}

fn encrypt(key: &str, message: &str) -> String {
    let crypt = new_magic_crypt!(key, 256);
    let b64_string = crypt.encrypt_str_to_base64(message);
    b64_string
}

fn decrypt(key: &str, b64_string: &str) -> Result<String> {
    let crypt = new_magic_crypt!(key, 256);

    match crypt.decrypt_base64_to_string(&b64_string) {
        Ok(msg) => Ok(msg),
        Err(_) => bail!("Failed to decrypt message!")
    }
}