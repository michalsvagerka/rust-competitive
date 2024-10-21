pub fn logceil(v: usize) -> usize {
    let mut bits = 1usize;
    while (1usize << bits) < v { bits += 1; }
    bits
}

pub fn upsize_to_power_of_two<T: Clone>(vec: &mut Vec<T>, fill: T) -> usize {
    let bits = logceil(vec.len());
    vec.resize(1usize << bits, fill);
    bits
}
