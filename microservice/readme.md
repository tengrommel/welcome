# Developing a Microservice with the Hyper Crate
> provide a short introduction to creating microservices using Rust the and 
hyper crate

***The hyper crate, which we'll use to compile the code, requires the OpenSSL library. The most 
popular operating systems already include the OpenSSL package and you can follow the manual of
 your package manager to install it***

## Binding a Tiny Server
> we'll create a Tiny Server from scratch. 
We'll start with the necessary dependencies, declare a main function, and then try to build and run it.

## Adding necessary dependencies
    
    cargo new hyper-microservice
    
    [dependencies]
    hyper = "0.12"
    
The single dependency is the hyper crate. The latest release of this crate is asynchronous and lies on top of the futures crate. It also uses the tokio crate for runtime, which includes the scheduler, reactor, and asynchronous sockets. Some of the necessary types of the tokio crate are re-exported in the hyper::rt module.<br>
 The main purpose of hyper is to operate with the HTTP protocol, which means that the crate can support other runtimes in the future.
 
## The main function of the server
> Let's start with the main function and add the necessary dependencies one by one, looking in detail at why we need each one. 

A minimal HTTP server needs the following:

- An address to bind to 
- A server instance to handle incoming requests
- A default handler for any request
- A reactor (runtime) where the server instance will operate