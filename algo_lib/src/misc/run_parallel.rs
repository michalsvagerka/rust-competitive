use crate::io::input::Input;
use crate::io::output::Output;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
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
