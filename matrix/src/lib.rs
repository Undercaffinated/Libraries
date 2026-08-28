#[allow(unused)]
#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}

#[allow(unused)]
impl<T> Matrix<T> {
    fn from(r: usize, c: usize, d: Vec<T>) -> Self {
        // Ensure the size of the given data will fit into a matrix
        // size rows x columns.
        assert!(r * c == d.len());
        Self {
            data: d,
            rows: r,
            columns: c,
        }
    }
}

#[allow(unused)]
impl<T: Clone> Matrix<T> {
    /// Returns the index of the element specified by (row, col) given self.
    fn index_from_row_col(&self, r: usize, c: usize) -> usize {
        r * self.columns + c
    }

    /// Calculates the (row, column) address of the nth element of self.data.
    fn row_col_from_index(&self, index: usize) -> (usize, usize) {
        let row: usize = index / self.columns;
        let col: usize = index % row;
        (row, col)
    }

    /// Returns a clone of the nth element in self.data.
    fn nth_element(&self, n: usize) -> T {
        self.data[n].clone()
    }

    /// Returns a clone of the element at (row, col) in self.data.
    fn rc_element(&self, r: usize, c: usize) -> T {
        self.nth_element(self.index_from_row_col(r, c)).clone()
    }

    /// Returns a new vector containing the elements in row r. Note, matrix rows, columsn, and data
    /// are zero-indexed.
    fn row(&self, r: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(self.columns);
        for i in 0..self.columns {
            v.push(self.data[r * self.columns + i].clone());
        }
        v
    }

    fn col(&self, c: usize) -> Vec<T> {
        let mut v: Vec<T> = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            v.push(self.data[i * self.columns + c].clone());
        }
        v
    }
}

impl<T: std::ops::Add<Output = T> + Copy + Clone> std::ops::Add for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Matrix<T> {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.columns, rhs.columns);
        let mut v: Vec<T> = Vec::with_capacity(self.data.len());

        for i in 0..self.data.len() {
            v.push(self.data[i] + rhs.data[i]);
        }

        Self {
            data: v,
            rows: self.rows,
            columns: self.columns,
        }
    }
}

impl<T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Clone> std::ops::Mul
    for Matrix<T>
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Matrix<T> {
        assert_eq!(self.columns, rhs.rows);
        let mut v: Vec<T> = Vec::with_capacity(self.rows * rhs.columns);

        Self {
            data: v,
            rows: self.rows,
            columns: self.columns,
        }
    }
}

/// ∀a:Vec<T>, ∀b:Vec<T>, dot_product(a,b) := Σ_(i=0)^n(a_i * b_i)
fn dot_product<T: std::ops::AddAssign + std::ops::Mul<Output = T> + Copy>(
    a: Vec<T>,
    b: Vec<T>,
) -> T {
    assert_eq!(a.len(), b.len());
    _sum_vec(_vec_product(a, b))
}

/// Given array a with data type T, _sum_vec := Σ_(i=0)^n(a_i)
fn _sum_vec<T: std::ops::AddAssign + Copy>(a: Vec<T>) -> T {
    let mut acc: T = a[1];
    if a.len() == 1 {
        return a[1];
    }
    for i in 1..a.len() {
        acc += a[i];
    }
    acc
}

/// For two arrays a and b with data type T where a.len() == b.len(), _vec_product := Vec::from([a_1 * b_1,
/// a_2 * b_2, ... a_n * b_n]).
fn _vec_product<T: Copy + std::ops::Mul<Output = T>>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut c: Vec<T> = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        c.push(a[i] * b[i]);
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let d1: Vec<i32> = Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let d2: Vec<i32> = d1.clone();
        let sum: Vec<i32> = Vec::from([2, 4, 6, 8, 10, 12, 14, 16, 18]);

        let m1: Matrix<i32> = Matrix::from(3, 3, d1);
        let m2: Matrix<i32> = Matrix::from(3, 3, d2);

        let m3 = m1 + m2;
        let m4 = Matrix::from(3, 3, sum);

        assert_eq!(m3, m4);
    }

    #[test]
    fn test_getters() {
        // Row getter
        let d: Vec<i32> = Vec::from([1, 2, 3, 4]);
        let m1: Matrix<i32> = Matrix::from(2, 2, d);
        assert_eq!(m1.row(0), Vec::from([1, 2]));
        assert_eq!(m1.row(1), Vec::from([3, 4]));

        // Column Getter
        assert_eq!(m1.col(0), Vec::from([1, 3]));
        assert_eq!(m1.col(1), Vec::from([2, 4]));
    }
}
