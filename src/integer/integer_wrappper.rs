use std::ops::Add;

const MOD: usize = 1usize << 32;

pub struct Wrap32 {
    raw_value: u32
}

impl Add<u32> for Wrap32 {
    type Output = Wrap32;

    fn add(self, other: u32) -> Wrap32 {
        Wrap32::new(self.raw_value + other)
    }
}

impl Wrap32 {

    pub fn new(value: u32) -> Self {
        Wrap32 {raw_value: value}
    }

    pub fn unwrap(&self, zero_point: &Wrap32, checkpoint: usize) -> usize {
        let abs1 = &self.raw_value as usize + checkpoint/MOD * MOD - &zero_point.raw_value as usize;
        if abs1  > checkpoint {
            if abs1 < MOD {
                abs1
            } else {
                let abs2 = abs1 - MOD;
                if checkpoint - abs2 < abs1 - checkpoint {abs2} else {abs1}
            }
        } else {
            let abs2 = abs1 + MOD;
            if checkpoint - abs1 < abs2 - checkpoint {abs1} else {abs2}
        }
    }

    pub fn wrap(n: usize, zero_point: &Wrap32) -> Wrap32 {
        zero_point + n as u32
    }
}