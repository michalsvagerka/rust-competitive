use std::ops::Range;

pub trait BinarySearchable<T>: PartialOrd {
    fn bisect(a: T, b: T) -> T;
    fn next(a: T) -> T;
    fn prev(a: T) -> T;
}

impl<T> BinarySearchable<T> for T
where
    T: PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::ops::Div<Output=T> + Copy + From<i8>,
{
    fn bisect(a: T, b: T) -> T {
        a + b / T::from(2i8)
    }

    fn next(a: T) -> T {
        a + T::from(1i8)
    }

    fn prev(a: T) -> T {
        a - T::from(1i8)
    }
}


pub trait BinarySearch<T> {
    fn lowest_index<F: Fn(T) -> bool>(self, f: F) -> Option<T>;
    fn highest_index<F: Fn(T) -> bool>(self, f: F) -> Option<T>;
}

impl<T> BinarySearch<T> for Range<T>
where
    T: PartialOrd + BinarySearchable<T> + Copy + From<i8>,
{
    fn lowest_index<F: Fn(T) -> bool>(self, f: F) -> Option<T> {
        let mut r = None;
        let mut l = self.start;
        let mut h = self.end;
        while l <= h {
            let m = T::bisect(l, h);
            if f(m) {
                h = T::prev(m);
                r = Some(m);
            } else {
                l = T::next(m);
            }
        }
        r
    }

    fn highest_index<F: Fn(T) -> bool>(self, f: F) -> Option<T> {
        let mut r = None;
        let mut l = self.start;
        let mut h = self.end;
        while l <= h {
            let m = T::bisect(l, h);
            if f(m) {
                l = T::next(m);
                r = Some(m);
            } else {
                h = T::prev(m);
            }
        }
        r
    }
}
