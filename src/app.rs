mod algos;

use algos::sha256;

/**
 * 実行
 */
pub fn exec() {
    println!("Hello, world!");


    let hash = sha256::get_hash("hello, world!");
    let hash_out = hash.unwrap();
    println!("{hash_out}");
}
