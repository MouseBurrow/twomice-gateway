use argon2::{
    password_hash, password_hash::{rand_core::OsRng, SaltString}, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier,
    Version,
};

const ARGON2_ALGORITHM: Algorithm = Algorithm::Argon2id;
const ARGON2_VERSION: Version = Version::V0x13;
const ARGON2_PARAMS: Params = Params::DEFAULT;

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2_def = Argon2::new(ARGON2_ALGORITHM, ARGON2_VERSION, ARGON2_PARAMS);
    let password_hash: PasswordHash = argon2_def.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: String) -> Result<(), password_hash::Error> {
    let argon2_hash = PasswordHash::new(hash.as_str())?;
    let argon2_def = Argon2::new(ARGON2_ALGORITHM, ARGON2_VERSION, ARGON2_PARAMS);
    argon2_def.verify_password(password.as_bytes(), &argon2_hash)
}
