use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, data: &PreCalc) {
    #[derive(Clone, Default)]
    struct Job {}

    impl ParallelJob<PreCalc> for Job {
        fn read_input(&mut self, input: &mut Input) {
            $CARET
        }

        fn solve(&mut self, data: &PreCalc) {}

        fn write_output(&mut self, out: &mut Output, test_case: usize) {
            out.print_line((format!("Case #{}:", test_case), ));
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
