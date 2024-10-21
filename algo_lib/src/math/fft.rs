use crate::math::modular::Field;
use crate::misc::logceil::upsize_to_power_of_two;

const MAX_BITS: usize = 25;

struct FFT<const M: u32> {
    omega: [Field<M>; MAX_BITS],
    omega_inv: [Field<M>; MAX_BITS],
    bits: usize,
}

impl<const M: u32> Default for FFT<M> {
    fn default() -> Self {
        match M {
            4194304001u32 => Self::new(25, 199, 758768563),
            998244353u32 => Self::new(23, 31, 128805723),
            104857601u32 => Self::new(22, 21, 49932191),
            924844033u32 => Self::new(21, 3597, 508059997),
            _ => { panic!("Unknown modulus, add root and root_inv manually"); }
        }
    }
}

impl<const M: u32> FFT<M> {
    pub fn new(bits: usize, root: u32, root_inv: u32) -> Self {
        assert!(bits <= MAX_BITS);

        let mut omega = [Field::<M>(root); MAX_BITS];
        let mut omega_inv = [Field::<M>(root_inv); MAX_BITS];
        for i in (0..bits - 1).rev() {
            omega[i] = omega[i + 1] * omega[i + 1];
            omega_inv[i] = omega_inv[i + 1] * omega_inv[i + 1];
        }
        Self {
            omega,
            omega_inv,
            bits,
        }
    }

    pub fn fft(self, vec: &mut Vec<Field<M>>) {
        let bit_size = upsize_to_power_of_two(vec, Field(0));
        assert!(bit_size <= self.bits, "Number of bits in FFT too large");
        Self::fft_sized(&self.omega, vec, bit_size)
    }

    pub fn fft_inverse(self, vec: &mut Vec<Field<M>>) {
        let bit_size = upsize_to_power_of_two(vec, Field(0));
        assert!(bit_size <= self.bits, "Number of bits in FFT too large");
        Self::fft_sized(&self.omega_inv, vec, bit_size);
        let q = Field(1 << bit_size).inv();
        for v in vec {
            *v *= q;
        }
    }

    fn fft_sized(omega: &[Field<M>; MAX_BITS], vec: &mut Vec<Field<M>>, bit_size: usize) {
        if bit_size == 0 { return; }
        let len = 1 << bit_size;
        assert_eq!(vec.len(), len, "Vector has wrong size");

        let mut j = 0;
        for i in 1..len {
            let mut bit = 1 << (bit_size - 1);
            while j >= bit {
                j -= bit;
                bit >>= 1;
            }
            j += bit;
            if i < j { vec.swap(i, j) }
        }

        for bit in 0..bit_size {
            let omega_b = omega[bit];
            let s = 1 << bit;
            for i in (0..len).step_by(2 * s) {
                let mut x = Field(1);
                for j in 0..s {
                    let k = i + j;
                    let l = k + s;
                    let u = vec[k];
                    let v = vec[l] * x;
                    vec[k] = u + v;
                    vec[l] = u - v;
                    x *= omega_b;
                }
            }
        }
    }
}
