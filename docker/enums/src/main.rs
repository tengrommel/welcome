// The enum keyword allows the creation of a type which may be one of
// a few different variants. Any variant which is valid as a struct is also
// valid as an enum
enum WebEvent {
    // An `enum` may either be `unit-like`
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i64, y:i64}
}

// A function which takes a `WebEvent` enum as an argument
// and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click {x, y} => println!("clicked at x={}, y={}", x, y),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}