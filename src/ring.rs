use std::{
    cmp,
    collections::HashMap,
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmallRing {
    module: u32,
}

impl std::fmt::Display for SmallRing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Ring (mod ")?;
        self.module.fmt(f)?;
        f.write_str(")")
    }
}

impl Ring for SmallRing {
    type Element = SmallRingElement;
    type Module = u32;
    type Value = u64;

    fn create_element<'a>(&self, value: Self::Value) -> Self::Element {
        SmallRingElement {
            ring: self.clone(),
            value,
        }
    }
    fn module(&self) -> &Self::Module {
        &self.module
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SmallRingElement {
    ring: SmallRing,
    value: u64,
}

impl Add for SmallRingElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.ring != rhs.ring {
            panic!(
                "Ring operation failed, lhs ring: {}, rhs ring: {}",
                self.ring, rhs.ring
            );
        }
        let value = (self.value + rhs.value) % (*self.ring.module() as u64);

        Self {
            ring: self.ring.clone(),
            value,
        }
    }
}

impl Sub for SmallRingElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.ring != rhs.ring {
            panic!(
                "Ring operation failed, lhs ring: {}, rhs ring: {}",
                self.ring, rhs.ring
            );
        }
        let value =
            (self.value + *self.ring.module() as u64 + rhs.value) % (*self.ring.module() as u64);

        Self {
            ring: self.ring.clone(),
            value,
        }
    }
}

impl Mul for SmallRingElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        if self.ring != rhs.ring {
            panic!(
                "Ring operation failed, lhs ring: {}, rhs ring: {}",
                self.ring, rhs.ring
            );
        }
        let value = (self.value * rhs.value) % (*self.ring.module() as u64);

        Self {
            ring: self.ring.clone(),
            value,
        }
    }
}

impl Rem for SmallRingElement {
    type Output = SmallRingElement;
    fn rem(self, rhs: Self) -> Self::Output {
        let value = self.value % rhs.value;
        self.ring.create_element(value)
    }
}

impl Neg for SmallRingElement {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.ring
            .create_element(*self.ring.module() as u64 - self.value)
    }
}

impl PartialOrd for SmallRingElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for SmallRingElement {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl std::fmt::Display for SmallRingElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
impl RingElement for SmallRingElement {
    fn ring(&self) -> &impl Ring {
        &self.ring
    }
}

#[derive(Debug, Clone)]
pub struct ExtendedEuclideanView {
    /// x = y * m + r
    down: Vec<[i64; 4]>,
    /// x * n + y * m
    up: Vec<[i64; 4]>,
}

pub fn extended_euclidean(x: i64, y: i64) -> Result<(i64, i64, ExtendedEuclideanView), i64> {
    let x = x.abs();
    let y = y.abs();
    let swapped = x < y;
    let (x, y) = if swapped { (x, y) } else { (y, x) };
    if y < 2 {
        return Err(y);
    }

    let mut down: Vec<[i64; 4]> = vec![];
    // x = ya + r
    let mut current_row = [x, y, 0, 0];
    while current_row[1] > 1 {
        current_row[2] = current_row[0] / current_row[1];
        current_row[3] = current_row[0] % current_row[1];
        // remainders
        down.push(current_row);
        current_row = [current_row[1], current_row[3], 0, 0];
    }
    if current_row[1] == 0 {
        // We found gcd > 1
        return Err(current_row[0]);
    }

    // We have last row in `down` x = ya + r, r = 1
    let last_step = down.last().expect("Infallible");

    let mut up: Vec<[i64; 4]> = vec![];
    // xa + yb = 1
    current_row = [last_step[0], 1, last_step[1], -last_step[2]];
    up.push(current_row);
    for [z, _, d, _] in down.iter().rev().skip(1) {
        // z = xd + y
        // xa + yb = 1
        // zb + x(a-bd) = 1
        current_row = [
            *z,
            current_row[3],
            current_row[0],
            current_row[1] - current_row[3] * d,
        ];
        up.push(current_row);
    }
    let view = ExtendedEuclideanView { down, up };
    let (a, b) = if swapped {
        (current_row[1], current_row[3])
    } else {
        (current_row[3], current_row[1])
    };
    Ok((a, b, view))
}

pub trait Ring: std::fmt::Debug + Clone + Send + Sync + 'static {
    type Element: RingElement;
    type Module;
    type Value;

    fn create_element(&self, value: Self::Value) -> Self::Element;
    fn module(&self) -> &Self::Module;
}

pub trait RingElement:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Neg
    + PartialOrd
    + Ord
    + Hash
    + std::fmt::Debug
    + std::fmt::Display
    + Clone
    + Copy
    + Send
    + Sync
    + 'static
{
    fn ring(&self) -> &impl Ring;
}
