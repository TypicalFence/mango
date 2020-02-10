#[cfg(feature = "aes")]
extern crate tiger_digest;

#[cfg(feature = "aes")]
use self::tiger_digest::{Digest, Tiger};

pub fn tiger_128(key: String) -> Vec<u8> {
    // generate a Tiger/128 hash
    // Tiger/128 is the first 128 bits of Tiger(/192)
    // http://www.cs.technion.ac.il/~biham/Reports/Tiger/tiger/node2.html
    let mut hasher = Tiger::default();
    hasher.input(&key.as_bytes());
    let mut hash = hasher.result().to_vec();
    // hash is 24 * 8 bits, 192 bit in total
    // 16 * 8 bits gives us the deisred 128 bits
    hash.split_off(16);
    return hash;
}

#[cfg(test)]
mod test {
    use super::tiger_128;

    #[test]
    fn len_128() {
        let hash = tiger_128(String::from("lol lol lol"));
        assert_eq!(hash.len(), 16);
    }
}
