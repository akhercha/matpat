mod glob;
mod glob_result;

use glob::glob;

fn main() {
    println!("{}", glob("[][-", "]"));
}
