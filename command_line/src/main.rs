extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
    message: String, // you define a struct named Options that has one String field called message. Then you annotate the struct with the custom derive attribute #[derive(StructOpt)]. This way StructOpt
    #[structopt(short = "d", long = "dead")]
    dead: bool,
    cat_file: Option<std::path::PathBuf>,
    // The type defined for cat_file is wrapped in an Option<T>.
    // This is how you indicate that this field is optional.
    // If the field is not provided, it will simply be an Option::None.
}

// Printing to STDERR
/**
Up until now, you have been printing using println!(), which print to
the standard output(STDOUT). However, there is also the standard error (STDERR)
stream that you can and should print errors to.
Rust provides a STDERR equivalent of println!(), called eprintln!().
*/

// Colored
/**
Coloring the text:red(), green(),blue(),etc
Coloring the background: on_red()(i.e, text on red background), on_green(), on_blue(), etc
Brighter version: bright_red()
Styling: bold(), underline(), italic()
*/

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead{"x"} else {"o"};
    println!("{}", message.bright_black().underline().on_yellow());
    match &options.cat_file {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .expect(&format!("could not read file {:?}", path));
            let cat_picture = cat_template.replace("{eye}", eye);
            println!("{}", &cat_picture);
        },
        None => {
            if message.to_lowercase() == "woof" {
                eprintln!("A cat shouldn't bark like a dog.")
            }
            println!(" \\");
            println!(" \\");
            println!(" /\\_/\\");
            println!(" ( {eye} {eye} )", eye=eye); // [2]
            println!(" =( I )=");
        }
        }
    }

