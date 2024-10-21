use std::iter::once;

pub fn prefix_sum<T: std::ops::Add<Output=T> + Copy>(v: impl Iterator<Item=T>, neutral: T) -> impl Iterator<Item=T> {
    prefix_custom(v, neutral, |a, b| {
        a + b
    })
}

pub fn prefix_product<T: std::ops::Mul<Output=T> + Copy>(v: impl Iterator<Item=T>, neutral: T) -> impl Iterator<Item=T> {
    prefix_custom(v, neutral, |a, b| {
        a * b
    })
}

pub fn prefix_custom<T: Copy, F: Fn(T, T) -> T>(v: impl Iterator<Item=T>, neutral: T, add: F) -> impl Iterator<Item=T> {
    once(neutral).chain(
        v.scan(neutral, move |sum, i| {
            *sum = add(*sum, i);
            Some(*sum)
        })
    )
}

pub fn suffix_custom<T: Copy, F: Fn(T, T) -> T>(v: impl Iterator<Item=T> + DoubleEndedIterator, neutral: T, add: F) -> impl Iterator<Item=T> {
    prefix_custom(v.rev(), neutral, add)
}

pub fn without<T: Copy, F: Fn(T, T) -> T + Copy>(v: &[T], neutral: T, add: F) -> Vec<T> {
    let n = v.len();
    let prefix = prefix_custom(v.iter().copied(), neutral, add).collect::<Vec<_>>();
    let suffix = prefix_custom(v.iter().copied().rev(), neutral, add).collect::<Vec<_>>();
    (0..n).map(|i|
        add(prefix[i], suffix[suffix.iter().len() - 2 - i])
    ).collect()
}

pub fn without_sum<T: Copy + std::ops::Add<Output=T>>(v: &[T], neutral: T) -> Vec<T> {
    without(v, neutral, std::ops::Add::add)
}

pub fn without_product<T: Copy + std::ops::Mul<Output=T>>(v: &[T], neutral: T) -> Vec<T> {
    without(v, neutral, std::ops::Mul::mul)
}
