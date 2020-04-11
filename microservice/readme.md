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

## Address of the server
> A socket address consists of an IP address and a port number.

The standard Rust library contains an IpAddr type to represent the IP address. <br>
We'll use the SocketAddr struct, which contains both the IpAddr and the u16 for the port number. 
We can construct the SocketAddr from a tuple of the ([u8; 4]) type. 
Add the following code to our main function:

    let addr = ([127, 0, 0, 1], 8080).into();

>We used an implementation of the impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr trait here,
which, in turn, uses impl From<[u8; 4]> for IpAddr. 
<br>
This lets us use the .into() method call to construct a socket address from the tuple.
<br> 
Similarly, we can create new SocketAddr instances with a constructor. 
In production applications, we will parse the socket addresses from external strings 
(command-line parameters or environment variables), and if no variants are set, 
we'll create SocketAddr from a tuple with default values.

## Server instances
> we can create a server instance and bind to this address:
    
    let builder = Server::bind(&addr);

> The preceding line creates a hyper::server::Server instance with a bind constructor that actually returns Builder, not a Server instance. The Server struct implements the Future trait.<br>
 It has similar role to Result, but describes a value that isn't available immediately.

## Setting the requests handler
> The Builder struct provides methods to tweak the parameters of the server created

- hyper's server supports both HTTP1 and HTTP2.
 
- You can use a builder value to choose either one protocol or both. 

> we're using builder to attach a service for handling incoming HTTP requests using the serve method:

    let server = builder.serve(|| {
        service_fn_ok(|_| {
            Response::new(Body::from("Almost microservice..."))
        })
    });
> Here, we're using the builder instance to attach a function that generates a Service instance. 
This function implements the hyper::service::NewService trait. 
The generated item then has to implement the hyper::service::Service trait. 
A service in a hyper crate is a function that takes a request and gives a response back. 
We haven't implemented this trait in this example; instead, we'll use the service_fn_ok function, 
which turns a function with suitable types into a service handler.

There are two corresponding structs: hyper::Request and hyper::Response. 

In the preceding code, we ignored a request argument and constructed the same response for every request. 

The response contains a body of static text.

## Adding the server instance to a runtime

> The runtime expects a Future instance with the Future<Item = (), Error = ()> type, but the Server struct implements a Future with the hyper::Error error type.

Drop an error from the server using the following:

    let server = server.map_err(drop);

We now have everything we need and can start the server with the specific runtime. Use the hyper::rt::run function to start the server:

    hyper::rt::run(server);

## Rebuilding on changes
> When you're working on developing web servers, it's useful to have instant access to compiled and running applications. 
<br>It's tiresome to have to restart cargo run manually whenever you change the code. I recommend that you install and use the cargo-watch subcommand on cargo. 
<br>This will monitor the changes made to the files of your project and restart the other commands you have chosen.

    cargo install cargo-watch
    cargo watch -x "run"

## Handling incoming request
> We've created a server, but it isn't very useful until it can respond on real requests.

we'll add handlers to the requests and use the principles of REST.

## Adding a service function
> As we saw previously, the Future trait has two associated types: 
one for a successful result and one for an error.

The service_fn function expects the result to be converted into future with the IntoFuture trait.

    let server = builder.serve(|| service_fn(microservice_handler));

Then add this unimplemented service function:

    fn microservice_handler(req: Request<Body>)
        -> impl Future<Item=Response<Body>, Error=Error>
    {
        unimplemented!();
    }

Similar to the previous one, this function expects a Request, 
but it doesn't return a simple Response instance. Instead, it returns a future result. 

Since Future is a trait(which doesn't have a size), we can't return an unsized entity from the 
function and we have to wrap it in a Box. However, in this case, we used a brand new approach, which 

## Implementing a service function

Out service function will support two kinds of requests:

- GET requests to the / path with an index page response
- Other requests with a NOT_FOUND response

## Implementing the REST principles
> If everyone were to create rules of interaction with microservices from scratch, we'd 
have an excess of private standards of intercommunication.

REST isn't a strict set of rules, but it's an architectural style intended to make interacting
with microservices simple.

- Adding a shared state
> You may have already heard that shared data is a bad thing and a potential cause of bottlenecks, 
if it has to be changed from separate threads. However, shared data can be useful if we want to 
share the address of a channel or if we don't need frequent access to it.