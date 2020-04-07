# welcome to rust

# 异步IO
> 所谓异步IO，是指以事件触发的机制来对IO操作进行处理

与多进程和多线程技术相比，异步I/O技术的最大优势是系统开销小，系统不必创建进程/线程，也不必维护这些进程/线程，从而大大减小了系统的开销。

# 以select方式实现高性能网络服务器
- 遍历文件描述符集中的所有描述符，找出有变化的描述符
- 对于侦听的socket和数据处理的socket要区别对待
- socket必须设置为非阻塞方式工作

# 重要API
- FD_ZERO、FD_SET、FD_ISSET
- flag fcntl(fd, F_SETFL/F_GETFL, flag)
- events select(fd, F_SETFL/F_GETFL, flag)

# 使用Epoll的好处
- 没有文件描述符的限制
- 工作效率不会随着文件描述符的增加而下降
- Epoll经历系统优化更高效
- Level Trigger没有处理反复发送
- Edge Trigger只发送一次

# Epoll重要的API

- int epoll_create() 参数无意义，可忽略
- int epoll_ctl(epfd, op, fd, struct epoll_event *event)
- int epoll_wait(epfd, events, maxevents, timeout)

# Epoll的事件
- EPOLLET
- EPOLLIN
- EPOLLOUT
- EPOLLPRI
- EPOLLERR
- EPOLLHUP

# epoll_ctl相关操作
- EPOLL_CTL_ADD
- EPOLL_CTL_MOD
- EPOLL_CTL_DEL

# Implement TCP

# System Comparisons

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
