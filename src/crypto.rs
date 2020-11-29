use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

pub fn getRandomSalt() -> Result<String, Unspecified> {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;
    Ok(HEXUPPER.encode(&salt))
}

//pub fn getPasswordHas(password: String, hash: String) -> Result<String, Unspecified> {
//
//    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
//    let n_iter = NonZeroU32::new(100_000).unwrap();
//    pbkdf2::derive(
//        pbkdf2::PBKDF2_HMAC_SHA512,
//        n_iter,
//        &salt,
//        password.as_bytes(),
//        &mut pbkdf2_hash,
//    );
//    Ok(HEXUPPER.encode(&pbkdf2_hsh))
//}
//
//pub fn verifyPassword(password: String, pbkdf2_hash: String salt: String) {
//    let n_iter = NonZeroU32::new(100_000).unwrap();
//    let should_succeed = pbkdf2::verify(
//        pbkdf2::PBKDF2_HMAC_SHA512,
//        n_iter,
//        &salt,
//        password.as_bytes(),
//        &pbkdf2_hash,
//    );
//}
