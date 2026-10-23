use std::ops::{Add, Sub};

/// A square `n` x `n` matrix of `f64` values.
#[derive(Debug, Clone, PartialEq)]
pub struct SquareMatrix {
    /// The size of the matrix
    n: usize,
    /// Entries stored in row-major form.
    data: Vec<f64>,
}

impl SquareMatrix {
    /// Creates an `n` x `n` matrix filled with zeros.
    pub fn new(n: usize) -> Self {
        SquareMatrix {n: n, data: vec![0.0; n * n]}
    }

    /// Returns the size of the matrix. 
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns the value at `row` and `col`.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.n + col]
    }

    /// Sets the value at `row` and `col` to `value`.
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.n + col] = value;
    }

    /// Splits the matrix into four `n/2` x `n/2` quadrants, returned as
    /// `(top_left, top_right, bottom_left, bottom_right)`.
    pub fn split(&self) -> (Self, Self, Self, Self) {
        let half = self.n / 2;
        let mut top_left = Vec::with_capacity(half * half);
        let mut top_right = Vec::with_capacity(half * half);
        let mut bottom_left = Vec::with_capacity(half * half);
        let mut bottom_right = Vec::with_capacity(half * half);

        // Iterate through each item in the array and based on the index, put it into proper sub array. 
        for (i, &value) in self.data.iter().enumerate() {
            let row = i / self.n;
            let col = i % self.n;
            match (row < half, col < half) {
                (true, true) => top_left.push(value),
                (true, false) => top_right.push(value),
                (false, true) => bottom_left.push(value),
                (false, false) => bottom_right.push(value),
            }
        }

        (
            SquareMatrix { n: half, data: top_left },
            SquareMatrix { n: half, data: top_right },
            SquareMatrix { n: half, data: bottom_left },
            SquareMatrix { n: half, data: bottom_right },
        )
    }

    /// Combines four equally sized quadrants back into a single matrix (the
    /// inverse of [`SquareMatrix::split`]).
    pub fn combine(top_left: &Self, top_right: &Self, bottom_left: &Self, bottom_right: &Self) -> Self {
        let half = top_left.n;
        let mut data = Vec::with_capacity(4 * half * half);

        // For the top half, go by column then by row
        for row in 0..half {
            for col in 0..half {
                data.push(top_left.get(row, col));
            }
            for col in 0..half {
                data.push(top_right.get(row, col));
            }
        }

        // Bottom half
        for row in 0..half {
            for col in 0..half {
                data.push(bottom_left.get(row, col));
            }
            for col in 0..half {
                data.push(bottom_right.get(row, col));
            }
        }

        SquareMatrix { n: 2 * half, data }
    }

    /// Multiplies `self` by `other` using conventional matrix multiplication.
    /// Returns `None` if the dimensions are incompatible.
    pub fn multiply(&self, other: &Self) -> Option<Self> {
        if self.n != other.n {
            return None;
        }

        let n = self.n;
        let mut result = SquareMatrix { n, data: vec![0.0; n * n] };

        // Go through each row of the result
        for row in 0..n {
            // Go through each column of the result
            for col in 0..n {
                // Walk across row `row` of self and down column `col` of other
                // at the same time, adding up the products of each pair
                let mut sum = 0.0;
                for k in 0..n {
                    sum += self.get(row, k) * other.get(k, col);
                }
                result.set(row, col, sum);
            }
        }

        Some(result)
    }

    /// Multiplies `self` by `other` using Strassen's algorithm.
    /// Returns `None` if the dimensions are incompatible.
    pub fn strassen_multiply(&self, other: &Self) -> Option<Self> {
        if self.n != other.n {
            return None;
        }

        // Base case: small matrices use standard multiplication.
        if self.n <= 2 {
            return self.multiply(other);
        }

        // Partition both matrices into quadrants:
        // M1: [[a b], [c d]], M2: [[e f] [g h]]
        let (a, b, c, d) = self.split();
        let (e, f, g, h) = other.split();

        // The 7 Strassen products, each a recursive call on half-size
        let m1 = (&a + &d).strassen_multiply(&(&e + &h))?;
        let m2 = (&c + &d).strassen_multiply(&e)?;
        let m3 = a.strassen_multiply(&(&f - &h))?;
        let m4 = d.strassen_multiply(&(&g - &e))?;
        let m5 = (&a + &b).strassen_multiply(&h)?;
        let m6 = (&c - &a).strassen_multiply(&(&e + &f))?;
        let m7 = (&b - &d).strassen_multiply(&(&g + &h))?;

        // Compute the quadrants of the result from the products
        let top_left = &(&(&m1 + &m4) - &m5) + &m7;
        let top_right = &m3 + &m5;
        let bottom_left = &m2 + &m4;
        let bottom_right = &(&(&m1 - &m2) + &m3) + &m6;

        Some(SquareMatrix::combine(&top_left, &top_right, &bottom_left, &bottom_right))
    }
}

impl Add for &SquareMatrix {
    type Output = SquareMatrix;

    /// Adds `self` and `other`.
    fn add(self, other: Self) -> SquareMatrix {
        assert_eq!(self.n, other.n, "can only add matrices of the same size");

        // Add the entries at each index
        let mut data = Vec::with_capacity(self.n * self.n);
        for i in 0..self.data.len() {
            data.push(self.data[i] + other.data[i]);
        }

        SquareMatrix { n: self.n, data }
    }
}

impl Sub for &SquareMatrix {
    type Output = SquareMatrix;

    /// Subtracts `other` from `self` entry by entry.
    fn sub(self, other: Self) -> SquareMatrix {
        assert_eq!(self.n, other.n, "can only subtract matrices of the same size");

        // Subtract the entries at each index
        let mut data = Vec::with_capacity(self.n * self.n);
        for i in 0..self.data.len() {
            data.push(self.data[i] - other.data[i]);
        }

        SquareMatrix { n: self.n, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set() {
        let mut m = SquareMatrix::new(3);
        assert_eq!(m.get(1, 2), 0.0);
        m.set(1, 2, 5.0);
        assert_eq!(m.get(1, 2), 5.0);
    }

    #[test]
    fn test_split_and_combine() {
        // [ 1  2  3  4]
        // [ 5  6  7  8]
        // [ 9 10 11 12]
        // [13 14 15 16]
        let m = SquareMatrix { n: 4, data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] };

        let (top_left, top_right, bottom_left, bottom_right) = m.split();
        assert_eq!(top_left.data, vec![1.0, 2.0, 5.0, 6.0]);
        assert_eq!(top_right.data, vec![3.0, 4.0, 7.0, 8.0]);
        assert_eq!(bottom_left.data, vec![9.0, 10.0, 13.0, 14.0]);
        assert_eq!(bottom_right.data, vec![11.0, 12.0, 15.0, 16.0]);

        // Combining the quadrants gives back the original matrix
        assert_eq!(SquareMatrix::combine(&top_left, &top_right, &bottom_left, &bottom_right), m);
    }

    #[test]
    fn test_add_sub() {
        let a = SquareMatrix { n: 2, data: vec![1.0, 2.0, 3.0, 4.0] };
        let b = SquareMatrix { n: 2, data: vec![5.0, 6.0, 7.0, 8.0] };
        assert_eq!((&a + &b).data, vec![6.0, 8.0, 10.0, 12.0]);
        assert_eq!((&b - &a).data, vec![4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_multiply() {
        let a = SquareMatrix { n: 2, data: vec![1.0, 2.0, 3.0, 4.0] };
        let b = SquareMatrix { n: 2, data: vec![5.0, 6.0, 7.0, 8.0] };
        assert_eq!(a.multiply(&b).unwrap().data, vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_strassen_multiply() {
        // 4x4 so Strassen splits once before reaching the 2x2 base case
        let a = SquareMatrix { n: 4, data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] };
        let b = SquareMatrix { n: 4, data: vec![2.0, 0.0, 1.0, 3.0, 1.0, 4.0, 0.0, 2.0, 3.0, 1.0, 2.0, 0.0, 0.0, 2.0, 1.0, 1.0] };
        assert_eq!(a.strassen_multiply(&b), a.multiply(&b));
    }

    #[test]
    fn test_mismatched_sizes() {
        let a = SquareMatrix::new(2);
        let b = SquareMatrix::new(4);
        assert_eq!(a.multiply(&b), None);
        assert_eq!(a.strassen_multiply(&b), None);
    }
}
