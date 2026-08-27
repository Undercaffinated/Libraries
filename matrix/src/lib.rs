
#[allow(unused)]
pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    columns: usize,
}


impl<T> Matrix<T> {
    fn from(r: usize, c: usize, d: Vec<T>) -> Self{
        // Ensure the size of the given data will fit into a matrix
        // size rows x columns.
        assert!(r * c > d.len());
        Self {
            data: d,
            rows: r,
            columns: c
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
}

impl<T: std::ops::Add> std::ops::Add for Matrix<T> {
    type Output = T;

    fn add(self, rhs: Self) -> T {
        
    }
}




// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_row_col_from_index() {
//     //     let a: Matrix<i32> = 
//     // }
// }



