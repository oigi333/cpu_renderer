use std::ops::{self, Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl_op_ex!(+ |lhs: &Vector4, rhs: &Vector4| -> Vector4 {
    Vector4 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
        w : lhs.w + rhs.w
    }
});

impl_op_ex!(- |lhs: &Vector4, rhs: &Vector4| -> Vector4 {
    Vector4 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
        w : lhs.w - rhs.w
    }
});

impl_op_ex!(* |lhs: &Vector4, rhs: &Vector4| -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z + lhs.w * rhs.w
});

impl_op_ex!(* |lhs: &Vector4, rhs: f64| -> Vector4 {
    Vector4 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
        w: lhs.w * rhs
    }
});

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl_op_ex!(+ |lhs: &Vector3, rhs: &Vector3| -> Vector3 {
    Vector3 {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z
    }
});

impl_op_ex!(- |lhs: &Vector3, rhs: &Vector3| -> Vector3 {
    Vector3 {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z
    }
});

impl_op_ex!(* |lhs: &Vector3, rhs: &Vector3| -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
});

impl_op_ex!(* |lhs: &Vector3, rhs: f64| -> Vector3 {
    Vector3 {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs
    }
});

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Vector2> for f64 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}
