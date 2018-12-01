#[macro_use] extern crate lazy_static;

lazy_static!{
    static ref FREQ_INPUTS: Vec<isize> = include_str!("../freq_input.txt").split('\n')
                                                     .map(|line| line.parse())
                                                     .filter_map(Result::ok)
                                                     .collect();
}

fn main() {
    let mut acc = 0;
    let mut already_found = vec![];
    for input in FREQ_INPUTS.iter().cycle() {
        acc += input;
        let mut herm = already_found.binary_search(&acc);
        if let Ok(_) = herm {
            break;
        } else if let Err(i) = herm {
            already_found.insert(i, acc);
        }
    }
    println!("{}", acc)
}
