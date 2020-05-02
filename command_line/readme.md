# Building a Command-Line Program
> Command-line programs, also known as CLIs(command-line interfaces), are one of the most natural applications of Rust. When you compile your first Hello World program, you are building a command-line program.

A typical command-line program takes arguments, flags, and sometimes standard input and then 
executes its main algorithm and output to the standard output or file.

All these operations are well supported by the Rust standard library and the third-party crates on crates.io.

There are a few advantages to building a CLI in Rust.
- First, the rich collection of libraries on crates.io will enable you to achieve many things you need
- Second, its high performance and safety guarantees let you mitigate many performance bottlenecks and 
bugs, compared to other popular scripting languages like python or Ruby
- Finally, Rust programs can be compiled into a single, small binary containing platform-specific machine 
code for easy distribution, so users don't need to have a language runtime on their systems

Cowsay is a funny little command-line program originally written in Perl.

It takes a text message and renders an ASCII-art cow(it looks more like a horse to me, to be honest) saying that 
message in a speech bubble. Although this program seems pretty useless, it's still quite popular on UNIX 
servers, where the system administrator can use it to print a light-hearted welcome message to the user.

Cowsay has a very simple algorithm, so by using it as an example, I can focus on 
the mechanisms and tooling to build a command-line program, instead of focusing on the business logic.

- Take a string as the positional argument
- Take a -d/--dead flag that makes the cat's eyes become xx, which is the comical expression of dead eyes
- Take a -h/--help flag to print a help message
- Take a -v/--version flag to print the version information
- Print the image in color
- Error handling: print any error message to STDERR
- Piping: accept STDIN as input and allow the output to be piped to other programs
- Run integration tests
- Package and publish the tool to crates.io

# Handling Complex Arguments with StructOpt
> The std::env::args function works well for small programs with only a few options.

But once you have more and more options, it becomes cumbersome to parse them by hand.

These types of arguments are prevalent in command-line tools, but implementing them from scratch every time is a real pain.

<b>To make your life even simpler, there is the great library called StructOpt that combines clap and custom derive. Custom derive is a feature in Rust that automatically generates a default implementation of a trait by annotating a struct.</b>
