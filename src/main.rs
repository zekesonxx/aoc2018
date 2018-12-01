#[macro_use] extern crate lazy_static;

lazy_static!{
    static ref FREQ_INPUTS: Vec<isize> = include_str!("../freq_input.txt").split('\n')
                                                     .map(|line| line.parse())
                                                     .filter_map(Result::ok)
                                                     .collect();
}

fn main() {
    let mut acc = 0;
    for input in FREQ_INPUTS.iter() {
        acc += input;
    }
    println!("{}", acc)
}
