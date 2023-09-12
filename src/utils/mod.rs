use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::Error;

pub fn hash_password(password: String) -> Result<String, Error> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Generate salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($argon2id$v=19$...) and return it
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Parse PHC string to PasswordHash struct
    let parsed_hash = PasswordHash::new(&password_hash)?;

    // Verify password against hash
    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(())
}

pub fn print_app_name(app_name: &str, mut len: usize, border: usize) {
    let mut num_spaces = (len - (border * 2) - app_name.len()) / 2;
    if num_spaces % 2 != 0 {
        num_spaces += 1;
        len += 1;
    }

    // print top border
    println!();
    for _ in 0..len {
        print!("#");
    }
    println!();

    // region: print app name row
    print!("##");
    for _ in 0..num_spaces {
        print!(" ");
    }

    print!("{app_name}");

    for _ in 0..num_spaces {
        print!(" ");
    }
    print!("##");
    println!();
    // endregion: print app name row

    // print bottom border
    for _ in 0..len {
        print!("#");
    }

    println!();
    println!();
}
