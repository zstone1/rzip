
#[cfg(test)]
mod tests {
    use crate::huffman::{huff_tree, encode, decode};
    #[test]
    fn full_loop() {
        let message = String::from("aa");
        let htree = huff_tree(&message);
        let encoded = encode(&htree, &message);
        let decoded = decode(&htree, &encoded);
        println!("htree: {:?}",htree);
        println!("encoded: {:?}",encoded);
        println!("decoded: {:?}",decoded);
        assert_eq!(decoded, message);
    }
}