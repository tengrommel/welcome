# welcome to rust
> example

# rust implement block chain

# rust Implement TCP
> trust

# welcome wasm

# openGL in wasm

## System Comparisons

- SVG 
    - Simple to learn
    - Poor performance
    - Thumbs down on 

- WebGL
    - Much more efficient than SVG
    - Very cross-platform(97%)
    - Steep learning curve

- WebGL2
    - More efficient than WebGL
    - Steep learning curve
    - Poor browser support (no IOS)

- WebGPU
    - Extremely efficient
    - Extremely steep learning curve
    - Extremely poor browser support

# Storing Efficiently
- Trade-offs considering speed and readability
- Accessing heap and stack variables
- How immutability influences design

# Storing Efficiently

## Heaps and stacks

Sized and unsized
> For the compiler to translate writen code into a binary format, it's necessary to know each type's size.

Slices work around the size issue by storing a fixed-size reference(&str) to the heap-allocated value, 
along with its length in bytes. Similar to pointers, this is a fixed-size view into a previously-unsized value.

## Generics
> Rust supports generics and even allows us to enforce the implementation of certain traits

These constraints can either come as a where clause attached to the function definition or with a colon 
in the generic type declaration

    fn my_generic_func<T: MyTrait>(t: T) {
        // code
    }
    fn my_generic_func<T>(t: T) where T: MyTrait {
        // code
    }
    // but better use in 2018 and beyond
    fn my_generic_func(t: impl MyTrait) {
        // code
    }

Additionally, the 2018 impl Trait syntax simplifies single-trait requirements(to do static instead of dynamic dispatch)
for input and return parameters, thereby eliminating the need for a Box or length type constrains (such as 
MyTrait in the preceding snippet)

When working with generics, the situation is a bit more complex. Type parameters are Sized by default (see 
the preceding snippet), which means that they will not match unsized types.

## Accessing the box
> An extra step doesn't sound like much, but it has considerable consequences.


# actix 