// https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/D?source=google
pub mod solution {
//{"name":"D: Splitting Hares","group":"Meta Coding Competitions - Meta Hacker Cup 2024 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2024/round-2/problems/D?source=google","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5\n1 2 7 -1 8\n2 2 1 1 1\n5\n4 1 2 -1 -1\n2 1 2 1 1\n5\n-1 -1 3 -1 5\n1 1 1 5 5\n6\n2 3 1 6 4 5\n1 1 1 2 2 2\n5\n1 -1 10 11 12\n1 1 1 2 2\n7\n7 2 -1 10 16 19 21\n1 1 2 2 2 3 3\n","output":"Case #1: Yes\n1 2 7 6 8\nCase #2: No\nCase #3: Yes\n2 1 3 4 5\nCase #4: No\nCase #5: Yes\n1 9 10 11 12\nCase #6: Yes\n7 2 13 10 16 19 21\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"splitting_hares_.*input[.]txt"},"output":{"type":"file","fileName":"splitting_hares_output.txt","pattern":null},"languages":{"java":{"taskClass":"DSplittingHares"}}}

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::algo_lib::misc::run_parallel::run_parallel;
use crate::algo_lib::misc::run_parallel::ParallelJob;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {
        n: usize,
        w: Vec<i32>,
        c: Vec<u32>,
        ok: bool,
    }

    impl ParallelJob<PreCalc> for Job {
        fn read_input(&mut self, input: &mut Input) {
            self.n = input.read();
            self.w = input.read_vec(self.n);
            self.c = input.read_vec(self.n);
            self.ok = false;
        }

        fn solve(&mut self, data: &PreCalc) {
            let mut known_weights = vec![];

            // 0 is weights of knowns
            // 1 is ids of unknowns
            let mut colors: HashMap<_, (Vec<_>, Vec<_>)> = HashMap::new();
            let mut processed_colors = HashSet::new();

            let mut used_weights = vec![false; 10000];
            used_weights[0] = true;

            for i in 0..self.n {
                // *color_count.entry(self.c[i]).or_default() += 1;

                if self.w[i] == -1 {
                    colors.entry(self.c[i]).or_default().1.push(i);
                } else {
                    let ww = self.w[i] as usize;
                    colors.entry(self.c[i]).or_default().0.push(ww);
                    known_weights.push((self.w[i], i));
                    used_weights[ww] = true;
                }
            }
            known_weights.sort();


            for cc in colors.values() {
                if cc.0.len() + cc.1.len() > 3 {
                    return;
                }
            }

            let mut last_color_processed = 0;
            for (weight, id) in known_weights {
                let color = self.c[id];
                if processed_colors.contains(&color) {
                    if color == last_color_processed {
                        // we already processed this color
                        continue;
                    } else {
                        // this is a skip, very bad
                        return;
                    }
                }

                last_color_processed = color;

                // process color fully

            }


            //
            // let mut intervals: Vec<(usize, usize, u32)> = vec![];
            // for (color, weights) in known_weights {
            //     let min = *weights.iter().min().unwrap();
            //     let max = *weights.iter().max().unwrap();
            //     intervals.push((min, max, color))
            // }
            // intervals.sort();
            // let mut last_used = 0;
            // let mut had_three = false;
            // for (lo, hi, color) in intervals {
            //     if lo <= last_used {
            //         // impossible, the intervals already overlap
            //         return;
            //     }
            //
            //
            //     for more in unknown_ids.entry(color).or_default() {
            //         while used_weights[last_used] {
            //             last_used += 1;
            //         }
            //         self.w[*more] = last_used as i32;
            //     }
            //
            //     last_used = max(last_used, hi);
            // }
            //
            // for ids in unknown_ids.values() {
            //     if !ids.is_empty() && self.w[ids[0]] == -1 {
            //         for id in ids {
            //             while used_weights[last_used] {
            //                 last_used += 1;
            //             }
            //             self.w[*id] = last_used as i32;
            //         }
            //     }
            // }
            // self.ok = true;
        }

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            if self.ok {
                out.print_line((format!("Case #{}: Yes", test_case),));
                for w in &self.w {
                    out.print(w);
                    out.print(' ');
                }
                out.print_line("");
            } else {
                out.print_line((format!("Case #{}: No", test_case),));
            }
        }
    }

    run_parallel::<Job, PreCalc>(input, output, data);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();
    solve(&mut input, &mut output, &pre_calc);
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


}
pub mod algo_lib {
pub mod io {
pub mod input {
use std::fmt::Debug;
use std::io::Read;
use std::str::FromStr;

pub struct Input<'s> {
    input: &'s mut dyn Read,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

macro_rules! read_impl {
    ($t: ty, $read_name: ident, $read_vec_name: ident) => {
        pub fn $read_name(&mut self) -> $t {
            self.read()
        }

        pub fn $read_vec_name(&mut self, len: usize) -> Vec<$t> {
            self.read_vec(len)
        }
    };

    ($t: ty, $read_name: ident, $read_vec_name: ident, $read_pair_vec_name: ident) => {
        read_impl!($t, $read_name, $read_vec_name);

        pub fn $read_pair_vec_name(&mut self, len: usize) -> Vec<($t, $t)> {
            self.read_vec(len)
        }
    };
}

impl<'s> Input<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(input: &'s mut dyn Read) -> Self {
        Self {
            input,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_with_size(input: &'s mut dyn Read, buf_size: usize) -> Self {
        Self {
            input,
            buf: vec![0; buf_size],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if res == b'\r' {
                if self.refill_buffer() && self.buf[self.at] == b'\n' {
                    self.at += 1;
                }
                return Some(b'\n');
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !char::from(b).is_whitespace() {
                return;
            }
            self.get();
        }
    }

    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if char::from(c).is_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    //noinspection RsSelfConvention
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(self.read());
        }
        res
    }

    pub fn read_string(&mut self) -> String {
        match self.next_token() {
            None => {
                panic!("Input exhausted");
            }
            Some(res) => unsafe { String::from_utf8_unchecked(res) },
        }
    }

    pub fn read_line(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            if c == b'\r' {
                if self.peek() == Some(b'\n') {
                    self.get();
                }
                break;
            }
            res.push(c.into());
        }
        res
    }

    fn read_integer<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let res = self.read_string();
        res.parse::<T>().unwrap()
    }


    pub fn read_char(&mut self) -> char {
        self.skip_whitespace();
        self.get().unwrap().into()
    }

    read_impl!(u32, read_unsigned, read_unsigned_vec);
    read_impl!(u64, read_u64, read_u64_vec);
    read_impl!(usize, read_size, read_size_vec, read_size_pair_vec);
    read_impl!(i32, read_int, read_int_vec, read_int_pair_vec);
    read_impl!(i64, read_long, read_long_vec, read_long_pair_vec);
    read_impl!(i128, read_i128, read_i128_vec);

    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = self.input.read(&mut self.buf).unwrap();
            self.buf_read != 0
        } else {
            true
        }
    }
}

pub trait Readable {
    fn read(input: &mut Input) -> Self;
}

impl Readable for char {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}

macro_rules! read_integer {
    ($($t:ident)+) => {$(
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_integer()
            }
        }
    )+};
}

read_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

macro_rules! tuple_readable {
    ($($name:ident)+) => {
        impl<$($name: Readable), +> Readable for ($($name,)+) {
            fn read(input: &mut Input) -> Self {
                ($($name::read(input),)+)
            }
        }
    }
}

tuple_readable! {T}
tuple_readable! {T U}
tuple_readable! {T U V}
tuple_readable! {T U V X}
tuple_readable! {T U V X Y}
tuple_readable! {T U V X Y Z}
tuple_readable! {T U V X Y Z A}
tuple_readable! {T U V X Y Z A B}
tuple_readable! {T U V X Y Z A B C}
tuple_readable! {T U V X Y Z A B C D}
tuple_readable! {T U V X Y Z A B C D E}
tuple_readable! {T U V X Y Z A B C D E F}
}
pub mod output {
use std::io::Write;

pub struct Output<'s> {
    output: &'s mut dyn Write,
    buf: Vec<u8>,
    at: usize,
    auto_flush: bool,
}

impl<'s> Output<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: false,
        }
    }

    pub fn new_with_auto_flush(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: true,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write_all(&self.buf[..self.at]).unwrap();
            self.output.flush().unwrap();
            self.at = 0;
            self.output.flush().expect("Couldn't flush output");
        }
    }

    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
    }

    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn maybe_flush(&mut self) {
        if self.auto_flush {
            self.flush();
        }
    }

    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        for i in arg {
            i.write(self);
            self.put(b'\n');
        }
    }

    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }
}

impl Write for Output<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        self.maybe_flush();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}

pub trait Writable {
    fn write(&self, output: &mut Output);
}

impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for &T {
    fn write(&self, output: &mut Output) {
        T::write(self, output)
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self.as_slice().write(output);
    }
}

macro_rules! write_to_string {
    ($($t:ident)+) => {$(
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    )+};
}

write_to_string!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! tuple_writable {
    ($name0:ident $($name:ident: $id:tt )*) => {
        impl<$name0: Writable, $($name: Writable,)*> Writable for ($name0, $($name,)*) {
            fn write(&self, out: &mut Output) {
                self.0.write(out);
                $(
                out.put(b' ');
                self.$id.write(out);
                )*
            }
        }
    }
}

tuple_writable! {T}
tuple_writable! {T U:1}
tuple_writable! {T U:1 V:2}
tuple_writable! {T U:1 V:2 X:3}
tuple_writable! {T U:1 V:2 X:3 Y:4}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6}

impl<T: Writable> Writable for Option<T> {
    fn write(&self, output: &mut Output) {
        match self {
            None => (-1).write(output),
            Some(t) => t.write(output),
        }
    }
}
}
}
pub mod misc {
pub mod run_parallel {
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPoolBuilder;
use std::sync::atomic::AtomicUsize;

pub trait ParallelJob<P>: Sync + Send + Default + Clone {
    fn read_input(&mut self, input: &mut Input);
    fn solve(&mut self, precalc: &P);
    fn write_output(&mut self, output: &mut Output, test_case: usize);
}

pub fn run_parallel<J: ParallelJob<P>, P: Send + Sync + 'static>(input: &mut Input, output: &mut Output, precalc: &P) {
    let t = input.read();
    let mut jobs = vec![J::default(); t];
    for job in jobs.iter_mut() {
        job.read_input(input);
    }
    ThreadPoolBuilder::new().num_threads(100)
        // .stack_size(1000000000)
        .build_global();
    // .unwrap();
    let rem = AtomicUsize::new(t);
    jobs.par_iter_mut().enumerate().for_each(|(test, job)| {
        job.solve(precalc);
        eprintln!(
            "Test {} done, {} remaining",
            test,
            rem.fetch_sub(1, std::sync::atomic::Ordering::Relaxed) - 1
        );
    });
    for (i, mut job) in jobs.into_iter().enumerate() {
        job.write_output(output, i + 1);
    }
}
}
}
}
fn main() {
    let paths = std::fs::read_dir(".").unwrap();
    let mut result = None;
    let mut last_accessed = None;
    let re = regex::Regex::new("splitting_hares_.*input[.]txt").unwrap();
    for path in paths {
        let path = path.unwrap();
        let cur_accessed = path.metadata().unwrap().accessed().unwrap();
        let path = path.path();
        let cur_name = path.file_name().unwrap().to_str().unwrap();
        if re.is_match(cur_name) {
            if last_accessed.is_none() || cur_accessed > last_accessed.unwrap() {
                result = Some(cur_name.to_string());
                last_accessed = Some(cur_accessed);
            }
        }
    }
    let mut in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = crate::algo_lib::io::input::Input::new(&mut in_file);
    let mut out_file = std::fs::File::create("splitting_hares_output.txt").unwrap();
    let output = crate::algo_lib::io::output::Output::new(&mut out_file);
    crate::solution::run(input, output);
}
 last_accessed = Some(cur_accessed);
            }
        }
    }
    let mut in_file = std::fs::File::open(result.unwrap()).unwrap();
    let input = crate::algo_lib::io::input::Input::new(&mut in_file);
    let mut out_file = std::fs::File::create("bunny_hopscotch_output.txt").unwrap();
    let output = crate::algo_lib::io::output::Output::new(&mut out_file);
    crate::solution::run(input, output);
}
