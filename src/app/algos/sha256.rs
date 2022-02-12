/// SHA256アルゴリズムでの処理
/// 
/// SHA256アルゴリズムでの処理を実装
/// 

use password_hash::Output;
use pbkdf2::{
    Pbkdf2,
    password_hash:: {
        SaltString,
        PasswordHasher
    }
};
use rand::rngs::OsRng;

/// ハッシュ値取得
/// 
/// sha256によるハッシュ値を取得する。
pub fn get_hash(base_value: &str) -> Option<Output> {

    let salt = SaltString::generate(&mut OsRng);

    // Hash password to PHC string ($pbkdf2-sha256$...)
    let hash_result = Pbkdf2.hash_password(base_value.as_bytes(), &salt);

    let hash = hash_result.unwrap();

    hash.hash
}