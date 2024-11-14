use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn v2(x: i64, y: i64) -> Vec2 {
    Vec2 { x, y }
}

pub fn v3(x: i64, y: i64, z: i64) -> Vec3 {
    Vec3 { x, y, z }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Vec2 {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self::new(0, 0);
    pub const E1: Self = Self::new(1, 0);
    pub const E2: Self = Self::new(0, 1);
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<&Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<&Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl SubAssign<&Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: &Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&i64> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<i64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&i64> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i64> for Vec2 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl MulAssign<&i64> for Vec2 {
    fn mul_assign(&mut self, rhs: &i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}
impl Neg for &Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl Vec2 {
    pub fn x_comp(&self) -> Vec2 {
        Vec2::new(self.x, 0)
    }

    pub fn y_comp(&self) -> Vec2 {
        Vec2::new(0, self.y)
    }

    pub fn dot(&self, rhs: Vec2) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn manhattan_dist(&self, rhs: Vec2) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    pub fn is_scaling(&self, other: Vec2) -> bool {
        self * other.x == other * self.x
    }

    pub fn manhattan_mag(&self) -> i64 {
        self.manhattan_dist(Vec2::ZERO)
    }

    pub fn neighbors_with_deltas<'a>(
        &self,
        deltas: &'a [Vec2],
    ) -> impl Iterator<Item = Vec2> + 'a {
        let s = *self;
        deltas.iter().map(move |delta| s + delta)
    }

    pub fn neighbors4(&self) -> impl Iterator<Item = Vec2> {
        const DELTAS: &[Vec2] = &[
            Vec2::new(1, 0),
            Vec2::new(-1, 0),
            Vec2::new(0, 1),
            Vec2::new(0, -1),
        ];

        self.neighbors_with_deltas(DELTAS)
    }

    pub fn neighbors8(&self) -> impl Iterator<Item = Vec2> {
        const DELTAS: &[Vec2] = &[
            Vec2::new(1, -1),
            Vec2::new(1, 0),
            Vec2::new(1, 1),
            Vec2::new(0, -1),
            Vec2::new(0, 1),
            Vec2::new(-1, -1),
            Vec2::new(-1, 0),
            Vec2::new(-1, 1),
        ];

        self.neighbors_with_deltas(DELTAS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&i64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<i64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&i64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<i64> for Vec3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl MulAssign<&i64> for Vec3 {
    fn mul_assign(&mut self, rhs: &i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Vec3 {
    pub fn dot(&self, rhs: Vec3) -> i64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: other.x * self.z - self.x * other.z,
            z: self.x * other.y - other.x * self.y,
        }
    }

    pub fn is_scaling(&self, other: Vec3) -> bool {
        self * other.x == other * self.x
    }

    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const E1: Self = Self::new(1, 0, 0);
    pub const E2: Self = Self::new(0, 1, 0);
    pub const E3: Self = Self::new(0, 0, 1);
}