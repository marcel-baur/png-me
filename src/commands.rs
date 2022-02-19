use crate::args::{DecodingArgs, EncodingArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use anyhow::{bail, Result};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::fs;
use std::str::FromStr;

pub fn encode(args: &EncodingArgs) -> Result<()> {
    println!("{:?}", args);
    match fs::read(args.path.clone()) {
        Ok(file) => {
            let mut png = Png::try_from(&file[..]).expect("Could not parse PNG from file.");
            let chunk_type = ChunkType::from_str(&args.chunk_type).expect("Creating chunk failed!");
            let data: Vec<u8> = match &args.key {
                Some(key) => encrypt(&key, &args.message).as_bytes().to_vec(),
                None => args.message.clone().as_bytes().to_vec(),
            };

            if !chunk_type.is_valid() {
                bail!("Invalid chunk type!");
            }
            let chunk = Chunk::new(chunk_type, data);
            png.append_chunk(chunk);
            match &args.output {
                Some(output) => fs::write(output.clone(), png.as_bytes())?,
                None => println!("{}", png),
            };
            Ok(())
        }
        Err(_) => {
            bail!("Could not read file!")
        }
    }
}

pub fn decode(args: &DecodingArgs) -> Result<()> {
    println!("{:?}", args);

    let file = fs::read(args.path.clone())?;
    let png = Png::try_from(&file[..]).expect("Failed to read PNG file");

    match png.chunk_by_type(&args.chunk_type) {
        Some(chunk) => {
            let data = chunk.data_as_string().expect("Failed to convert chunk data to string.");
            println!("Data: {}", data);
            let msg = match &args.key {
                Some(key) => decrypt(key, &data)?,
                None => data,
            };

            println!("The message is: {}", msg)
        },
        None => bail!("Chunk type not found!"),
    };

    Ok(())
}

fn encrypt(key: &str, message: &str) -> String {
    let crypt = new_magic_crypt!(key, 256);
    let b64_string = crypt.encrypt_str_to_base64(message);
    b64_string
}

fn decrypt(key: &str, b64_string: &str) -> Result<String> {
    let crypt = new_magic_crypt!(key, 256);
    println!("Decrypting {} with {}", key, b64_string);
    match crypt.decrypt_base64_to_string(&b64_string) {
        Ok(msg) => Ok(msg),
        Err(_) => bail!("Failed to decrypt message!"),
    }
}
