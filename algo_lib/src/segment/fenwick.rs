use crate::misc::logceil::logceil;

pub struct Fenwick<T> {
    len: usize,
    vec: Vec<T>,
}

impl<T: Default + Copy + std::ops::AddAssign> From<Vec<T>> for Fenwick<T> {
    fn from(mut vec: Vec<T>) -> Self {
        let len = 1usize << logceil(vec.len());
        vec.resize_with(len, T::default);
        for i in 0..len {
            let j = i + lsb(i + 1);
            if j < len {
                let f_i = vec[i];
                vec[j] += f_i;
            }
        }
        Self { len, vec }
    }
}

impl<T: Default + std::ops::AddAssign + Copy> Fenwick<T> {
    pub fn new(mut len: usize) -> Self {
        len = 1usize << logceil(len);
        let mut vec = Vec::new();
        vec.resize_with(len, T::default);
        Self { len, vec }
    }

    pub fn add(&mut self, mut i: usize, v: T) {
        while i <= self.len {
            self.vec[i] += v;
            i += lsb(i + 1);
        }
    }

    pub fn sum(&self, mut i: usize) -> T {
        let mut sum = T::default();
        while i > 0 {
            sum += self.vec[i - 1];
            i -= lsb(i);
        }
        sum
    }

    pub fn set(&mut self, i: usize, mut v: T)
    where
        T: std::ops::SubAssign,
    {
        v -= self.get(i);
        self.add(i, v)
    }

    pub fn get(&self, i: usize) -> T
    where
        T: std::ops::SubAssign,
    {
        self.range(i, i)
    }

    pub fn range(&self, mut i: usize, mut j: usize) -> T
    where
        T: std::ops::SubAssign,
    {
        j += 1;
        let mut s = T::default();
        while j > i {
            s += self.vec[j - 1];
            j -= lsb(j);
        }
        while i > j {
            s -= self.vec[i - 1];
            i -= lsb(i);
        }
        s
    }
}

const fn lsb(i: usize) -> usize {
    let i = i as i32;
    (i & -i) as usize
}
