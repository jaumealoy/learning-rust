use std::ops::{Add, Sub, Mul};
use std::fmt::{Display, Result};
use std::{panic, vec};
use std::thread::{self, JoinHandle};
use std::sync::{Mutex, Arc};
use std::clone::Clone;

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

    pub fn mul(&self, rhs: &Matrix) -> Matrix {
        // MxN * NxP -> MxP
        if (self.columns != rhs.columns) {
            panic!("Cannot multiply matrices: dimensions are not compatible");
        }

        // we could spawn a thread for each cell in the new matrix, but that would be a lot of
        // thread overhead and it would not increase the parallelism
        let mut result = Matrix::new(self.rows, rhs.columns);
        
        let mut threads = Vec::new();

        let arc_a = Arc::new(self.clone());
        let arc_b = Arc::new(rhs.clone());
        let data = Arc::new(Mutex::new(vec![vec![0; result.rows]; result.columns]));

        // A * B = C
        // Cij = sum A
        for row in 0..result.rows {
            for column in 0..result.columns {
                let a = arc_a.clone();
                let b = arc_b.clone();
                let res = data.clone();

                let thread = std::thread::spawn(move || {
                    let mut value: T = 0;
                    for i in 0..a.columns {
                        value += a.get(row, i) * b.get(i, column);
                    }
                    res.lock().unwrap()[row][column] = value;
                });
                threads.push(thread);
            }
        }

        for t in threads {
            t.join();
        }

        result.data = data.lock().unwrap().to_vec();

        result
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

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns && self.rows != rhs.columns {
            panic!("Cannot sum matrices: dimensions are not compatible");
        }

        let mut matrix = Matrix::new(self.columns, self.rows);

        for row in 0..self.rows {
            for column in 0..self.columns {
                matrix.set(row, column, self.get(row, column) + rhs.get(row, column));
            }
        }

        matrix
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.columns != rhs.columns && self.rows != rhs.columns {
            panic!("Cannot subtract matrices: dimensions are not compatible");
        }

        let mut matrix = Matrix::new(self.columns, self.rows);

        for row in 0..self.rows {
            for column in 0..self.columns {
                matrix.set(row, column, self.get(row, column) - rhs.get(row, column));
            }
        }

        matrix
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        // MxN * NxP -> MxP
        if (self.columns != rhs.columns) {
            panic!("Cannot multiply matrices: dimensions are not compatible");
        }

        let mut result = Matrix::new(self.rows, rhs.columns);

        // A * B = C
        // Cij = sum A
        for row in 0..result.rows {
            for column in 0..result.columns {
                let mut value: T = 0;
                for i in 0..self.columns {
                    value += self.get(row, i) * rhs.get(i, column);
                }
                result.set(row, column, value);
            }
        }

        result
    }
}

impl Clone for Matrix {
    fn clone(&self) -> Self {
        Matrix { 
            rows: self.rows.clone(), 
            columns: self.columns.clone(), 
            data: self.data.clone() 
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.rows = source.rows;
        self.columns = source.columns;
        self.data = source.data.clone();
    }
}