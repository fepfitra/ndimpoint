use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Point<T> {
    p: Vec<T>,
}

impl<T> Point<T>
where
    T: Into<f64> + Copy, // Ensures T can be converted to f64
{
    pub fn new(p: Vec<T>) -> Self {
        Point { p }
    }

    pub fn dim(&self) -> usize {
        self.p.len()
    }

    pub fn dist(&self) -> f64 {
        self.p.iter().map(|&x| x.into().powi(2)).sum::<f64>().sqrt()
    }

    pub fn apply(&self, func: fn(&[T]) -> f64) -> f64 {
        func(&self.p)
    }
}

// Implementing Add, Sub, Mul for Point<T>
impl<T> Add<&Point<T>> for &Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, other: &Point<T>) -> Self::Output {
        let p = self
            .p
            .iter()
            .zip(other.p.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Point { p }
    }
}

impl<T> Sub<&Point<T>> for &Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, other: &Point<T>) -> Self::Output {
        let p = self
            .p
            .iter()
            .zip(other.p.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Point { p }
    }
}

impl<T> Mul<&Point<T>> for &Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;

    fn mul(self, other: &Point<T>) -> Self::Output {
        let p = self
            .p
            .iter()
            .zip(other.p.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        Point { p }
    }
}

// Implementing scalar operations
impl<T> Add<T> for &Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a + scalar).collect();
        Point { p }
    }
}

impl<T> Sub<T> for &Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a - scalar).collect();
        Point { p }
    }
}

impl<T> Mul<T> for &Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a * scalar).collect();
        Point { p }
    }
}

impl<T> Div<T> for &Point<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Point<T>;

    fn div(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a / scalar).collect();
        Point { p }
    }
}

// Ownership operations
impl<T> Add<T> for Point<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Point<T>;

    fn add(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a + scalar).collect();
        Point { p }
    }
}

impl<T> Sub<T> for Point<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Point<T>;

    fn sub(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a - scalar).collect();
        Point { p }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;

    fn mul(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a * scalar).collect();
        Point { p }
    }
}

impl<T> Div<T> for Point<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Point<T>;

    fn div(self, scalar: T) -> Self::Output {
        let p = self.p.iter().map(|&a| a / scalar).collect();
        Point { p }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let ivec: Vec<i32> = vec![1, 2, 3];
        let fvec: Vec<f64> = vec![1.0, 2.0, 3.0];

        let iv = Point::new(ivec.clone());
        let fv = Point::new(fvec.clone());
        assert_eq!(iv.p, ivec);
        assert_eq!(fv.p, fvec);
        dbg!(iv);
    }

    #[test]
    fn add() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = Point::new(vec![4, 5, 6]);
        let iv3 = &iv1 + &iv2;
        assert_eq!(iv3.p, vec![5, 7, 9]);

        let fv1 = Point::new(vec![1.0, 2.0, 3.0]);
        let fv2 = Point::new(vec![4.0, 5.0, 6.0]);
        let fv3 = &fv1 + &fv2;
        assert_eq!(fv3.p, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn sub() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = Point::new(vec![4, 5, 6]);
        let iv3 = &iv1 - &iv2;
        assert_eq!(iv3.p, vec![-3, -3, -3]);
    }

    #[test]
    fn mul() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = Point::new(vec![4, 5, 6]);
        let iv3 = &iv1 * &iv2;
        assert_eq!(iv3.p, vec![4, 10, 18]);
    }

    #[test]
    fn scalar_add() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = &iv1 + 10;
        assert_eq!(iv2.p, vec![11, 12, 13]);
    }

    #[test]
    fn scalar_sub() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = &iv1 - 10;
        assert_eq!(iv2.p, vec![-9, -8, -7]);
    }

    #[test]
    fn scalar_mul() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = &iv1 * 10;
        assert_eq!(iv2.p, vec![10, 20, 30]);
    }

    #[test]
    fn scalar_div() {
        let iv1 = Point::new(vec![10, 20, 30]);
        let iv2 = &iv1 / 10;
        assert_eq!(iv2.p, vec![1, 2, 3]);
    }

    #[test]
    fn own_add() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = iv1.clone() + 10;
        assert_eq!(iv2.p, vec![11, 12, 13]);
    }

    #[test]
    fn own_sub() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = iv1.clone() - 10;
        assert_eq!(iv2.p, vec![-9, -8, -7]);
    }

    #[test]
    fn own_mul() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = iv1.clone() * 10;
        assert_eq!(iv2.p, vec![10, 20, 30]);
    }

    #[test]
    fn own_div() {
        let iv1 = Point::new(vec![10, 20, 30]);
        let iv2 = iv1.clone() / 10;
        assert_eq!(iv2.p, vec![1, 2, 3]);
    }

    #[test]
    fn dim() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = Point::new(vec![4, 5, 6, 7]);
        assert_eq!(iv1.dim(), 3);
        assert_eq!(iv2.dim(), 4);
    }

    #[test]
    fn dist() {
        let iv1 = Point::new(vec![1, 2, 3]);
        assert_eq!(iv1.dist(), (14.0_f64).sqrt());
    }

    #[test]
    fn apply() {
        let iv1 = Point::new(vec![1, 2, 3]);
        let iv2 = iv1.apply(|x| x[0] as f64 + x[1] as f64 + x[2] as f64);
        assert_eq!(iv2, 6.0);
    }
}
