mod glob;

use glob::glob;

fn main() {
    println!("{}", glob("[][-", "]"));
}
