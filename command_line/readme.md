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
message in a speech bubble 
