use rustler::{Encoder, Env, Error, Term};
use rustler::types::binary::{Binary, OwnedBinary};
use rustler::types::Atom;
use secp256k1::{SecretKey, PublicKey};

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
        atom error;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.RustySecp256k1",
    [
        ("ec_pubkey_create", 2, ec_pubkey_create, rustler::SchedulerFlags::DirtyCpu),
        ("ec_pubkey_tweak_add", 2, ec_pubkey_tweak_add, rustler::SchedulerFlags::DirtyCpu),
        ("ec_pubkey_decompress", 1, ec_pubkey_decompress, rustler::SchedulerFlags::DirtyCpu)
    ],
    None
}

fn ec_pubkey_tweak_add<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let public_key_binary: Binary = args[0].decode()?;

    if public_key_binary.len() < 33 {
       return Ok((atoms::error(), "Public key length is not 33 bits." ).encode(env)); 
    }

    let public_key_slice = public_key_binary.as_slice();
    let mut public_key_fixed: [u8; 33] = [0; 33];
    public_key_fixed.copy_from_slice(&public_key_slice[0..33]);
    let mut public_key = PublicKey::parse_compressed(&public_key_fixed).expect("Error parsing public key");

    let tweak_key_binary: Binary = args[1].decode()?;

    if tweak_key_binary.len() < 32 {
       return Ok((atoms::error(), "Tweak key is not 32 bits." ).encode(env)); 
    }

    let tweak_key_slice = tweak_key_binary.as_slice();
    let mut tweak_key_fixed: [u8; 32] = [0; 32];
    tweak_key_fixed.copy_from_slice(&tweak_key_slice[0..32]);
    let tweak_key = SecretKey::parse(&tweak_key_fixed).expect("Error parsing tweak key");

    PublicKey::tweak_add_assign(&mut public_key, &tweak_key).unwrap(); 

    let mut erl_bin: OwnedBinary = OwnedBinary::new(33).unwrap();
    let public_key_serialized = public_key.serialize_compressed();
    erl_bin.as_mut_slice().copy_from_slice(&public_key_serialized);

    Ok((atoms::ok(), erl_bin.release(env)).encode(env)) 
}

fn ec_pubkey_decompress<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let public_key_binary: Binary = args[0].decode()?;

    if public_key_binary.len() < 33 {
       return Ok((atoms::error(), "Public key length is not 33 bits." ).encode(env)); 
    }

    let public_key_slice = public_key_binary.as_slice();
    let mut public_key_fixed: [u8; 33] = [0; 33];
    public_key_fixed.copy_from_slice(&public_key_slice[0..33]);
    let public_key = PublicKey::parse_compressed(&public_key_fixed)
        .expect("Error parsing public key");

    let mut erl_bin: OwnedBinary = OwnedBinary::new(65).unwrap();
    let public_key_serialized = public_key.serialize();
    erl_bin.as_mut_slice().copy_from_slice(&public_key_serialized);

    Ok((atoms::ok(), erl_bin.release(env)).encode(env)) 
}

fn ec_pubkey_create<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let private_key_binary: Binary = args[0].decode()?;

    if private_key_binary.len() < 32 {
       return Ok((atoms::error(), "Private key length is less than 32 bits" ).encode(env)); 
    }

    let private_key_slice = private_key_binary.as_slice();
    let mut private_key_fixed: [u8; 32] = [0; 32];
    private_key_fixed.copy_from_slice(&private_key_slice[0..32]);

    let private_key = SecretKey::parse(&private_key_fixed).expect("Error parsing private key");

    let compression: Atom = args[1].decode()?;

    let uncompressed_type: Atom = match Atom::from_str(env, "uncompressed") {
        Ok(term) => term,
        _ => panic!("Unknown compression")
    };

    if compression == uncompressed_type {
      let public_key = PublicKey::from_secret_key(&private_key).serialize().to_vec();
      let mut public_key_fixed: [u8; 65] = [0; 65];
      public_key_fixed.copy_from_slice(&public_key.as_slice()[0..65]);

      let mut erl_bin: OwnedBinary = OwnedBinary::new(65).unwrap();
      erl_bin.as_mut_slice().copy_from_slice(&public_key);
      Ok((atoms::ok(), erl_bin.release(env)).encode(env))
    } else {
      let public_key = PublicKey::from_secret_key(&private_key).serialize_compressed().to_vec();
      let mut public_key_fixed: [u8; 33] = [0; 33];
      public_key_fixed.copy_from_slice(&public_key.as_slice()[0..33]);

      let mut erl_bin: OwnedBinary = OwnedBinary::new(33).unwrap();
      erl_bin.as_mut_slice().copy_from_slice(&public_key);
      Ok((atoms::ok(), erl_bin.release(env)).encode(env))
    }

}
