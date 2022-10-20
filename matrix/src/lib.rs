use std::fmt::Debug;
use std::ops::Mul;

struct Matrix {
    n: usize,
    m: usize,
    data: Vec<Vec<i32>>,
}

impl Matrix {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            data: vec![vec![0_i32; n]; m],
        }
    }

    fn from(mat: Vec<Vec<i32>>) -> Self {
        Self {
            n: mat.len(),
            m: mat[0].len(),
            data: mat,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.m != rhs.n {
            panic!("Cant multiple this matrix");
        }

        let mut res = Matrix::new(self.n, rhs.m);

        for i in 0..self.n {
            for j in 0..rhs.m {
                for t in 0..self.m {
                    res.data[i][j] += self.data[i][t] * rhs.data[t][j]
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matrix_mul() {
        let m1 = Matrix::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let m2 = Matrix::from(vec![vec![7, 8], vec![9, 10], vec![11, 12]]);
        let res = m1 * m2;
        assert_eq!(vec![vec![58, 64], vec![139, 154]], res.data);
    }

    #[test]
    fn matrix_mul_2() {
        let m1 = Matrix::from(vec![vec![4], vec![5], vec![6]]);
        let m2 = Matrix::from(vec![vec![1, 2, 3]]);
        let res = m1 * m2;
        assert_eq!(
            vec![vec![4, 8, 12], vec![5, 10, 15], vec![6, 12, 18]],
            res.data
        );
    }
}
