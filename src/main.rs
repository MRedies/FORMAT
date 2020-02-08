use std::fs;
extern crate clap;
use clap::App;

fn main() {
    App::new("FORmat")
       .version("0.1")
       .about("Improves the readability of Fortran code")
       .author("Matthias Redies")
       .get_matches();
    // --snip--
    let filename = "bla.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
