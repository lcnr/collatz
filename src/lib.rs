pub struct Sequence {
    sequence: u128,
    length: u8,
}

pub fn prev_power(x: u128) -> u128 {
    // to prevent copying
    struct W(bool);

    let mut n = 1;
    let mut carry = W(false);
    let mut prev_bit = W(true);
    for bit in 1..128 {
        match (x & 1 << bit != 0, carry, prev_bit) {
            (false, W(false), W(false)) => {
                prev_bit = W(false);
                carry = W(false);
            },
            (false, W(false), W(true)) |
            (false, W(true), W(false)) |
            (true, W(true), W(true)) => {
                n |= 1 << bit;
                prev_bit = W(true);
                carry = W(true);
            }
            (false, W(true), W(true)) => {
                carry = W(true);
                prev_bit = W(false);
            }
            (true, W(false), W(false)) => {
                n |= 1 << bit;
                prev_bit = W(true);
                carry = W(false);
            }
            (true, W(false), W(true)) |
            (true, W(true), W(false)) => {
                prev_bit = W(false);
                carry = W(false);
            }
        }
    }

    n
}

pub fn first_bits(x: u128, n: u8) -> u128 {
    x & (!0u128).checked_shr(128 - n as u32).unwrap_or(0)
}

impl Sequence {
    pub fn new(sequence: u128, length: u8) -> Self {
        assert_eq!(first_bits(sequence, length), sequence);

        Self { sequence, length }
    }

    pub fn pop_front(&mut self) {
        if self.length != 0 {
            self.length -= 1;
            self.sequence <<= 1;
        }
    }

    pub fn pop_back(&mut self) {
        if self.length != 0 {
            self.length -= 1;
            self.sequence = first_bits(self.sequence, self.length);
        }
    }

    pub fn at(&self, pos: u8) -> bool {
        self.sequence & 1 << pos != 0
    }

    pub fn start(&self) -> u128 {
        let mut pos = self.length;

        let mut expected = 0;
        let mut base = 1;
        while let Some(new_pos) = pos.checked_sub(1) {
            pos = new_pos;

            expected <<= 1;
            base <<= 1;
            //println!("{} o {} at {}({})", expected, base, self.at(pos), pos);
            if self.at(pos) {
                if expected == 0 {
                    expected += base;
                }

                expected -= 1;

                while expected % 3 != 0 {
                    expected += base;
                }
                expected /= 3;
            }
        }

        let res = first_bits(expected, self.length);
        assert_eq!(expected, res);
        //println!("exp: {:03b}", expected);
        expected
    }

    pub fn y(&self) -> u128 {
        let mut y: u128 = 0;
        let mut pow = 1;
        for i in 0..self.length {
            if self.at(i) {
                pow = prev_power(pow);
                y = y.wrapping_add(pow << i);
            }
        }

        first_bits(y, self.length)
    }

    pub fn is_valid(&self) -> bool {
        let lhs = (!0u128).wrapping_mul(self.start());
        //println!("lhs: {:03b}", first_bits(lhs, self.length));
        //println!("rhs: {:03b}", self.y());
        first_bits(lhs, self.length) == self.y()
    }
}
