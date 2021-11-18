use borsh::{BorshDeserialize, BorshSerialize};
extern crate base64;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    name: String,
    symbol: String,
    decimals: u8,
}

fn test_simple_struct() {
    let a = A {
        name: "ChainLink Token".to_string(),
        symbol: "LINK".to_string(),
        decimals: 18,
    };

    let encoded_a: Vec<u8> = a.try_to_vec().unwrap();
    //let decoded_a = A::try_from_slice(&encoded_a).unwrap();
    //assert_eq!(a, decoded_a);
    //println!("{:?}", decoded_a);
    println!("{:?}", base64::encode(encoded_a));
}

fn main() {
    //println!("Hello, world!");
    test_simple_struct();
}
