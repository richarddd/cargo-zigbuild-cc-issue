extern crate ring;

use ring::rand::{SecureRandom, SystemRandom};

fn main() {
    let mut nonce = vec![0; 12];

    // Fill nonce with random data
    let rand = SystemRandom::new();
    rand.fill(&mut nonce).unwrap();

    println!("{:?}", nonce);
}

//add tests for random
#[test]
fn test_random() {
    let mut nonce = vec![0; 12];

    // Fill nonce with random data
    let rand = SystemRandom::new();
    rand.fill(&mut nonce).unwrap();

    assert_ne!(nonce, vec![0; 12]);
}
