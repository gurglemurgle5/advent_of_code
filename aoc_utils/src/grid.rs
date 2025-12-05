use std::{
    iter::repeat,
    ops::{Index, IndexMut, Range},
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Box<[T]>,
}

impl<'a, T> Grid<T> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn row(&self, row: usize) -> &[T] {
        let width = self.width();
        assert!(row < self.height());
        &self.data[(row * width)..(row * width + width)]
    }

    pub fn rows(&'a self) -> Rows<'a, T> {
        Rows {
            rows: 0..self.height(),
            grid: self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rows<'a, T> {
    rows: Range<usize>,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let data = self.grid.row(self.rows.start);
            self.rows.start += 1;
            Some(data)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = self.rows.len();
        (left, Some(left))
    }
}

impl<'a, T> ExactSizeIterator for Rows<'a, T> {}

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.rows.is_empty() {
            let data = self.grid.row(self.rows.end - 1);
            self.rows.end -= 1;
            Some(data)
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let width = self.width();
        let height = self.height();
        assert!(index.0 < width);
        assert!(index.1 < height);
        &self.data()[index.1 * width + index.0]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let width = self.width();
        let height = self.height();
        assert!(index.0 < width);
        assert!(index.1 < height);
        &mut self.data_mut()[index.1 * width + index.0]
    }
}

impl FromStr for Grid<char> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(|line| line.chars().count()).max().ok_or(())?;
        let height = s.lines().count();
        let data: Box<[char]> = s
            .lines()
            .flat_map(|line| line.chars().chain(repeat(' ')).take(width))
            .collect();
        assert_eq!(data.len(), width * height);
        Ok(Grid {
            width,
            height,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "AaBb\nCcDd\nEeFf";

    #[test]
    fn test_width_height() {
        let grid: Grid<char> = TEST_DATA.parse().unwrap();
        assert_eq!(grid.width(), 4);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn test_uneven_lines() {
        let data = "AaBb\nCcDd    \nEeFf";
        let grid: Grid<char> = data.parse().unwrap();
        assert_eq!(grid.width(), 8);
        assert_eq!(grid.height(), 3);
        for row in grid.rows() {
            assert_eq!(row.len(), 8);
        }
    }

    #[test]
    fn test_index() {
        let grid: Grid<char> = TEST_DATA.parse().unwrap();
        assert_eq!(grid[(0, 0)], 'A');
        assert_eq!(grid[(1, 0)], 'a');
        assert_eq!(grid[(0, 1)], 'C');
        assert_eq!(grid[(3, 2)], 'f');
    }

    #[test]
    fn test_rows() {
        let grid: Grid<char> = TEST_DATA.parse().unwrap();
        let mut rows = grid.rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.next(), Some(['A', 'a', 'B', 'b'].as_slice()));
        assert_eq!(rows.len(), 2);
        assert_eq!(rows.next(), Some(['C', 'c', 'D', 'd'].as_slice()));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows.next(), Some(['E', 'e', 'F', 'f'].as_slice()));
        assert_eq!(rows.len(), 0);
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn test_rows_reverse() {
        let grid: Grid<char> = TEST_DATA.parse().unwrap();
        let mut rows = grid.rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.next_back(), Some(['E', 'e', 'F', 'f'].as_slice()));
        assert_eq!(rows.len(), 2);
        assert_eq!(rows.next_back(), Some(['C', 'c', 'D', 'd'].as_slice()));
        assert_eq!(rows.len(), 1);
        assert_eq!(rows.next_back(), Some(['A', 'a', 'B', 'b'].as_slice()));
        assert_eq!(rows.len(), 0);
        assert_eq!(rows.next_back(), None);
    }
}
