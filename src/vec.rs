use std::ops::{Add, Div, Mul, Sub};

pub type Point3D = Vec3D;

#[derive(Debug, Clone, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec3D {
        self / self.length()
    }

    pub fn format_color(&self) -> Vec3D {
        Vec3D {
            x: self.x * 255.999,
            y: self.y * 255.999,
            z: self.z * 255.999,
        }
    }
}

impl Add for Vec3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'this, 'other> Add<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn add(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'this, 'other> Sub<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn sub(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3D {
    type Output = Vec3D;

    fn mul(self, other: Self) -> Vec3D {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'this, 'other> Mul<&'other Vec3D> for &'this Vec3D {
    type Output = Vec3D;

    fn mul(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl<'other> Mul<&'other Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, other: &'other Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<'this> Mul<f64> for &'this Vec3D {
    type Output = Vec3D;

    fn mul(self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f64> for Vec3D {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<'this> Div<f64> for &'this Vec3D {
    type Output = Vec3D;

    fn div(self, scalar: f64) -> Vec3D {
        Vec3D {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_3d_dot_product_borrow() {
        let a = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vec3D {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            &a + &b,
            Vec3D {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        assert_eq!(
            &a - &b,
            Vec3D {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        assert_eq!(
            &a * &b,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            &a * 2.0,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            &b / 2.0,
            Vec3D {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn test_point_3d_dot_product_move() {
        let a = Vec3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vec3D {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(
            a + b,
            Vec3D {
                x: 3.0,
                y: 3.0,
                z: 3.0
            }
        );
        assert_eq!(
            a - b,
            Vec3D {
                x: -1.0,
                y: -1.0,
                z: -1.0
            }
        );
        assert_eq!(
            a * b,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            a * 2.0,
            Vec3D {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
        assert_eq!(
            b / 2.0,
            Vec3D {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }
}
