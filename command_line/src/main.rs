extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    message: String, // you define a struct named Options that has one String field called message. Then you annotate the struct with the custom derive attribute #[derive(StructOpt)]. This way StructOpt
    #[structopt(short = "d", long = "dead")]
    dead: bool,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead{"x"} else {"o"};
    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!(" /\\_/\\");
    println!(" ( {eye} {eye} )", eye=eye); // [2]
    println!(" =( I )=");
}
