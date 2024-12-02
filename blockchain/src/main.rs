use sha2::{Sha256, Digest};

fn main() {
    // The input data to hash (e.g., a string)
    let data = b"hello, world";

    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Feed the data into the hasher
    hasher.update(data);

    // Get the result as a byte array
    let result = hasher.finalize();

    // Convert the result to a hex string for easy viewing
    let hash_hex = format!("{:x}", result);

    println!("SHA-256 Hash: {}", hash_hex);
}
