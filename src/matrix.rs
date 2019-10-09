use std::ops;

use crate::vector::Vector4;

#[derive(Default, Clone, Copy, Debug)]
pub struct Matrix4 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m03: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m30: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}

impl Matrix4 {
    pub fn as_rows(&self) -> [Vector4; 4] {
        [
            Vector4::new(self.m00, self.m10, self.m20, self.m30),
            Vector4::new(self.m01, self.m11, self.m21, self.m31),
            Vector4::new(self.m02, self.m12, self.m22, self.m32),
            Vector4::new(self.m03, self.m13, self.m23, self.m33),
        ]
    }

    pub fn as_columns(&self) -> [Vector4; 4] {
        [
            Vector4::new(self.m00, self.m01, self.m02, self.m03),
            Vector4::new(self.m10, self.m11, self.m12, self.m13),
            Vector4::new(self.m20, self.m21, self.m22, self.m23),
            Vector4::new(self.m30, self.m31, self.m32, self.m33),
        ]
    }
}

impl_op_ex!(+ |lhs: &Matrix4, rhs: &Matrix4| -> Matrix4 {
    Matrix4 {
        m00: lhs.m00 + rhs.m00, m01: lhs.m01 + rhs.m01, m02: lhs.m02 + rhs.m02, m03: lhs.m03 + rhs.m03,
        m10: lhs.m10 + rhs.m10, m11: lhs.m11 + rhs.m11, m12: lhs.m12 + rhs.m12, m13: lhs.m13 + rhs.m13,
        m20: lhs.m20 + rhs.m20, m21: lhs.m21 + rhs.m21, m22: lhs.m22 + rhs.m22, m23: lhs.m23 + rhs.m23,
        m30: lhs.m30 + rhs.m30, m31: lhs.m31 + rhs.m31, m32: lhs.m32 + rhs.m32, m33: lhs.m33 + rhs.m33,
    }
});

impl_op_ex!(- |lhs: &Matrix4, rhs: &Matrix4| -> Matrix4 {
    Matrix4 {
        m00: lhs.m00 - rhs.m00, m01: lhs.m01 - rhs.m01, m02: lhs.m02 - rhs.m02, m03: lhs.m03 - rhs.m03,
        m10: lhs.m10 - rhs.m10, m11: lhs.m11 - rhs.m11, m12: lhs.m12 - rhs.m12, m13: lhs.m13 - rhs.m13,
        m20: lhs.m20 - rhs.m20, m21: lhs.m21 - rhs.m21, m22: lhs.m22 - rhs.m22, m23: lhs.m23 - rhs.m23,
        m30: lhs.m30 - rhs.m30, m31: lhs.m31 - rhs.m31, m32: lhs.m32 - rhs.m32, m33: lhs.m33 - rhs.m33,
    }
});

impl_op_ex!(* |lhs: &Matrix4, rhs: &Matrix4| -> Matrix4 {
    let rows = lhs.as_rows();
    let columns = rhs.as_columns();
    Matrix4 {
        m00: columns[0] * rows[0], m01: columns[1] * rows[1], m02: columns[0] * rows[2], m03: columns[0] * rows[3],
        m10: columns[1] * rows[0], m11: columns[1] * rows[1], m12: columns[1] * rows[2], m13: columns[1] * rows[3],
        m20: columns[2] * rows[0], m21: columns[2] * rows[1], m22: columns[2] * rows[2], m23: columns[2] * rows[3],
        m30: columns[3] * rows[0], m31: columns[3] * rows[1], m32: columns[3] * rows[2], m33: columns[3] * rows[3],
    }
});

impl_op_ex!(* |lhs: &Matrix4, rhs: &Vector4| -> Vector4 {
    let rows = lhs.as_rows();
    Vector4::new(
        rows[0] * rhs,
        rows[1] * rhs,
        rows[2] * rhs,
        rows[3] * rhs,
    )
});