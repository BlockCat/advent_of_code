use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Debug, Clone)]
pub struct BitSet {
    size: usize,
    bits: Vec<u64>,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            bits: vec![0; 64usize.div_ceil(size)],
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.size {
            panic!("Out of bounds");
        }

        let part = self.bits[index / 64];
        let index = index % 64;

        (part >> index) & 1 == 1
    }

    pub fn set(&mut self, index: usize, bit: bool) {
        if index >= self.size {
            panic!("Out of bounds");
        }
        if bit {
            self.bits[index / 64] |= 1 << (index % 64);
        } else {
            self.bits[index / 64] &= !(1 << (index % 64));
        }
    }
}

impl BitAnd for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            size: self.size,
            bits: self
                .bits
                .into_iter()
                .zip(rhs.bits.into_iter())
                .map(|(a, b)| a & b)
                .collect(),
        }
    }
}

impl BitOr for BitSet {
    type Output = BitSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            size: self.size,
            bits: self
                .bits
                .into_iter()
                .zip(rhs.bits.into_iter())
                .map(|(a, b)| a.bitor(b))
                .collect(),
        }
    }
}

impl BitXor for BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        Self {
            size: self.size,
            bits: self
                .bits
                .into_iter()
                .zip(rhs.bits.into_iter())
                .map(|(a, b)| a.bitxor(b))
                .collect(),
        }
    }
}
