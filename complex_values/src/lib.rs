//Implement with generic instead of only i128
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex(pub f64, pub f64);

impl Complex {

    /// Gives the absolut value of the complex number
    pub fn abs(self) -> f64{
        (self.0*self.0 + self.1*self.1).sqrt()
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex(
            self.0*rhs.0 - self.1*rhs.1,
            self.0*rhs.1 + self.1*rhs.0
        )
    }
    
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_multiplication() {
        let z1 = Complex(10.0, 2.0);
        let z2 = Complex(3.0, 53.0);
        let z3 = z1*z2;

        assert_eq!(z3, Complex(-76.0, 536.0));
    }

    #[test]
    fn complex_addition() {
        let z1 = Complex(10.0, 2.0);
        let z2 = Complex(3.0, 53.0);
        let z3 = z1+z2;

        assert_eq!(z3, Complex(13.0, 55.0));
    }
}
