use data_encoding::{BASE64URL, HEXUPPER};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
const SESSION_KEY_LEN: usize = 64;

pub fn get_random_salt() -> Result<String, Unspecified> {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;
    Ok(HEXUPPER.encode(&salt))
}

pub fn get_password_hash(password: &str, salt: &str) -> Result<String, Unspecified> {
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    let n_iter = NonZeroU32::new(100_000).unwrap();
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &HEXUPPER.decode(salt.as_bytes()).unwrap(),
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    Ok(HEXUPPER.encode(&pbkdf2_hash))
}

pub fn gen_session_key() -> Result<String, Unspecified> {
    let rng = rand::SystemRandom::new();

    let mut key = [0u8; SESSION_KEY_LEN];
    rng.fill(&mut key)?;
    Ok(BASE64URL.encode(&key))
}

pub fn verify_password(password: &str, pbkdf2_hash: &str, salt: &str) -> Result<(), Unspecified> {
    let n_iter = NonZeroU32::new(100_000).unwrap();
    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &HEXUPPER.decode(salt.as_bytes()).unwrap(),
        password.as_bytes(),
        &HEXUPPER.decode(pbkdf2_hash.as_bytes()).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_password_hash() {
        use super::*;

        let salt = "7BB1FC37C9D4EFE98DEB4244BA2B39DCA3202E7F1789CE57022FD889C8A0210D9E39FBE776176D7C32D1447C83773A4E1D2B28EC938DE542950B9CBAC4D4DC26";
        let hash = "A3456BB427E28F3B175E95B53E5A76D1418DA55B729EB399A891D2165A4C5DF7269F28E21B57656D016B6EB74DBF7BB59D67FE68B0DE6445C2A5FBA18BB667D1";

        match get_password_hash("hello123", salt) {
            Ok(v) => assert_eq!(v, hash),
            Err(_) => {
                panic!("Fail to create password hash");
            }
        };
    }

    #[test]
    fn test_verify_password() {
        use super::*;

        let salt = "7BB1FC37C9D4EFE98DEB4244BA2B39DCA3202E7F1789CE57022FD889C8A0210D9E39FBE776176D7C32D1447C83773A4E1D2B28EC938DE542950B9CBAC4D4DC26";
        let hash = "A3456BB427E28F3B175E95B53E5A76D1418DA55B729EB399A891D2165A4C5DF7269F28E21B57656D016B6EB74DBF7BB59D67FE68B0DE6445C2A5FBA18BB667D1";
        let password = "hello123";

        match verify_password(password, hash, salt) {
            Ok(_) => {}
            Err(_) => {
                panic!("Fail to verify password hash");
            }
        };

        let badpassword = "badpassword";
        match verify_password(badpassword, hash, salt) {
            Ok(_) => {
                panic!("Should fail with bad password");
            }
            Err(_) => {}
        };
    }
}
