use std::ops::{Add, Sub};
use std::fmt::{Display, Result};

type T = i64;

pub struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<Vec<T>>
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Self {
        let data = vec![vec![0; rows]; columns];

        Matrix {
            rows,
            columns,
            data
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut matrix = Matrix::new(n, n);
        
        for i in 0..n {
            matrix.set(i, i, 1);
        }

        matrix
    }

    pub fn set(&mut self, row: usize, column: usize, value: T) {
        self.data[row][column] = value;
    }

    pub fn get(&self, row: usize, column: usize) -> &T {
        &self.data[row][column]
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for column in 0..self.columns {
                write!(f, "{}\t", self.data[row][column]);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}