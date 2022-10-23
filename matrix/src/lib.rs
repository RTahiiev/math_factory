use std::{ops::Mul, result, vec};

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

    fn gauss(self) -> Vec<f32> {
        if self.n + 1 != self.m {
            panic!("Its a problem!")
        }

        let mut matrix: Vec<Vec<f32>> = vec![vec![0.0_f32; self.m]; self.n];
        for (i, vector) in self.data.iter().enumerate() {
            for (j, v) in vector.iter().enumerate() {
                matrix[i][j] = *v as f32;
            }
        }
        for i in 0..self.n {
            let mut best: usize = i;
            for j in 0..self.n {
                if matrix[j][i].abs() > matrix[best][i].abs() {
                    best = j;
                }
            }
            matrix.swap(best, i);
            for j in (0..self.m).rev() {
                matrix[i][j] = (matrix[i][j] / matrix[i][i]).floor();
            }
            for j in 0..self.n {
                if j != i {
                    for k in (0..self.m).rev() {
                        matrix[j][k] -= matrix[i][k] * matrix[j][i];
                    }
                }
            }
        }
        let mut result: Vec<f32> = vec![0.0_f32; self.n];
        for i in 0..self.n {
            result[i] = matrix[i][self.n];
        }
        result
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

fn binpow(mut base: i32, mut exponent: i32) -> f64 {
    let mut result: f64 = 1.0;
    let mut flag = false;
    if exponent < 0 {
        flag = true;
    }
    exponent = exponent.abs();
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = result * base as f64;
        }
        base *= base;
        exponent >>= 1;
    }
    if flag {
        return 1.0 / result;
    }
    result
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

    #[test]
    fn matrix_gauss() {
        let m1 = Matrix::from(vec![vec![1, 1, 3], vec![3, -2, 4]]);
        let res = m1.gauss();
        assert_eq!(vec![2.0, 1.0], res);
    }

    #[test]
    fn binpow_test() {
        let answer_1 = binpow(2, 4);
        let answer_2 = binpow(3, 3);
        let answer_3 = binpow(4, -2);
        assert_eq!(16.0, answer_1);
        assert_eq!(27.0, answer_2);
        assert_eq!(1.0 / 16.0, answer_3);
    }
}
