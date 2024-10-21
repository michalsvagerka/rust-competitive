//$JSON

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

$SOLVE

//START MAIN
mod tester;

fn main() {
    let name = "TODO";
    if name != "TODO" {
        let mut in_file = std::fs::File::open(&format!("C:\\Users\\micha\\Downloads\\{}_input.txt", name)).unwrap();
        let mut out_file = std::fs::File::create(&format!("C:\\Users\\micha\\Downloads\\{}_output.txt", name)).unwrap();
        run(Input::new(&mut in_file), Output::new(&mut out_file));
    } else {
        tester::run_tests();
    }
}
//END MAIN
