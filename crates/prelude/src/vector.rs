use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

impl Vector2 {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self::new(0, 0);
    pub const E1: Self = Self::new(1, 0);
    pub const E2: Self = Self::new(0, 1);
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<&Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: &Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<&Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Sub<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: &Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<i64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&i64> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<i64> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
impl Mul<&i64> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i64> for Vector2 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl MulAssign<&i64> for Vector2 {
    fn mul_assign(&mut self, rhs: &i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}
impl Neg for &Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}

impl Vector2 {
    pub fn x_comp(&self) -> Vector2 {
        Vector2::new(self.x, 0)
    }

    pub fn y_comp(&self) -> Vector2 {
        Vector2::new(0, self.y)
    }

    pub fn dot(&self, rhs: Vector2) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn manhatan_dist(&self, rhs: Vector2) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    pub fn manhatan_mag(&self) -> i64 {
        self.manhatan_dist(Vector2::ZERO)
    }

    pub fn neighbors_with_deltas<'a>(
        &self,
        deltas: &'a [Vector2],
    ) -> impl Iterator<Item = Vector2> + 'a {
        let s = *self;
        deltas.iter().map(move |delta| s + delta)
    }

    pub fn neighbors4(&self) -> impl Iterator<Item = Vector2> {
        const DELTAS: &[Vector2] = &[
            Vector2::new(1, 0),
            Vector2::new(-1, 0),
            Vector2::new(0, 1),
            Vector2::new(0, -1),
        ];

        self.neighbors_with_deltas(DELTAS)
    }

    pub fn neighbors8(&self) -> impl Iterator<Item = Vector2> {
        const DELTAS: &[Vector2] = &[
            Vector2::new(1, -1),
            Vector2::new(1, 0),
            Vector2::new(1, 1),
            Vector2::new(0, -1),
            Vector2::new(0, 1),
            Vector2::new(-1, -1),
            Vector2::new(-1, 0),
            Vector2::new(-1, 1),
        ];

        self.neighbors_with_deltas(DELTAS)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3 {
    pub const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<i64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&i64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &i64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<i64> for Vector3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl MulAssign<&i64> for Vector3 {
    fn mul_assign(&mut self, rhs: &i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
impl Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Vector3 {
    pub fn dot(&self, rhs: Vector3) -> i64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const E1: Self = Self::new(1, 0, 0);
    pub const E2: Self = Self::new(0, 1, 0);
    pub const E3: Self = Self::new(0, 0, 1);
}
