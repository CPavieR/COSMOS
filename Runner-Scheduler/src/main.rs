use std::env;

fn main() {
    if (env::args().len())==1 {
        panic!("Error: no semantic file receive")
    };
    if (env::args().len())>=3 {
        panic!("Error: too many argument")
    };
}
