pub trait FloatCompare {
    fn minf(self) -> f64;

    fn maxf(self) -> f64;
}

impl<T: Iterator<Item=f64>> FloatCompare for T {
    fn minf(self) -> f64 {
        self.fold(f64::INFINITY, |a, b| a.min(b))
    }

    fn maxf(self) -> f64 {
        self.fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }
}
