// fn say_hell() {
//     println!("hello");
// }
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", say_hello());
}
