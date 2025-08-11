use std::hash::{Hash, Hasher};

pub struct MHash {
    prev: u8,
    state: u64,
}

impl Default for MHash {
    fn default() -> Self {
        Self {
            prev: Default::default(),
            state: Default::default(),
        }
    }
}

impl Hasher for MHash {
    fn finish(&self) -> u64 {
        self.state
    }

    // I'm not gonna implement a real Hasher, that's for cryptographers
    // just using prime numbers and some btwise ops to jumble things
    // so the hash of "ABC" is never the same as "CBA"
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            let pb = (b ^ self.prev) as u64;
            // I'll use constants to mix then avalanche
            self.state ^= pb.wrapping_mul(0x9e3779b97f4a7c15);
            self.state = self.state.rotate_left(27).wrapping_mul(0x94d049bb133111eb);
            self.prev = *b;
        }
    }
}

pub fn hash<T: Hash>(seed: u64, t: T) -> u64 {
    let mut h = MHash::default();
    h.write_u64(seed);
    t.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_hasher() {
        let n = hash(55, "cat");
        assert_ne!(n, hash(55, "tac"));
    }

    #[test]
    pub fn test_numbers() {
        let mut prev = 0;
        for x in 0..10000 {
            let curr = hash(55, x);
            assert!(curr != prev);
            prev = curr;
        }
    }
}
