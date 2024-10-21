use std::mem::swap;

struct UnionFind {
    data: Vec<i32>,
    components: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self { data: vec![-1; n], components: n }
    }

    pub fn find(&mut self, i: usize) -> usize {
        assert!(i < self.data.len());

        let mut j = i as i32;
        unsafe {
            while *self.data.get_unchecked(j as usize) >= 0 {
                j = *self.data.get_unchecked(j as usize);
            }
        }
        let ans = j;

        // path compression
        j = i as i32;
        unsafe {
            while *self.data.get_unchecked(j as usize) >= 0 {
                let tmp = j;
                j = *self.data.get_unchecked(j as usize);
                *self.data.get_unchecked_mut(tmp as usize) = ans;
            }
        }

        ans as usize
    }

    pub fn size(&mut self, i: usize) -> usize {
        let f = self.find(i);
        unsafe { -self.data.get_unchecked(f) as usize }
    }

    pub fn united(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    /// Unite components `x` and `y`. Returns `None` if they were already united, and `Some((a,b))`
    /// when we merged `b` into `a`.
    pub fn unite(&mut self, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut i = self.find(i);
        let mut j = self.find(j);
        if i == j {
            None
        } else {
            self.components -= 1;
            unsafe {
                let mut i_size = -self.data.get_unchecked(i);
                let mut j_size = -self.data.get_unchecked(i);
                if i_size < j_size {
                    swap(&mut i, &mut j);
                    swap(&mut i_size, &mut j_size);
                }

                *self.data.get_unchecked_mut(i) -= j_size;
            }

            Some((i, j))
        }
    }
}
