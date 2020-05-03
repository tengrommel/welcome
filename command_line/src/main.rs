extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    message: String, // you define a struct named Options that has one String field called message. Then you annotate the struct with the custom derive attribute #[derive(StructOpt)]. This way StructOpt
    #[structopt(short = "d", long = "dead")]
    dead: bool,
}

// Printing to STDERR
/**
Up until now, you have been printing using println!(), which print to
the standard output(STDOUT). However, there is also the standard error (STDERR)
stream that you can and should print errors to.
Rust provides a STDERR equivalent of println!(), called eprintln!().
*/

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead{"x"} else {"o"};
    println!("{}", message); // get the message
    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.")
    }
    println!(" \\");
    println!(" \\");
    println!(" /\\_/\\");
    println!(" ( {eye} {eye} )", eye=eye); // [2]
    println!(" =( I )=");
}
