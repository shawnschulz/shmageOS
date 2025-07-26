# Description

Idea is to have a kernel centered around parallel programming in rust using partitioned address spaces. No preemptive multitasking, no processes. But does have multithreading for a single application, memory paging, and abstractions for distributed programming by creating a partitioned global address space abstraction across multiple machines that have the OS. Idea is you either just link against the library and boot/run from a container, nanovm or vm, OR you link against a mini-kernel image and boot into the OS itself, with the OS doing relatively minimal things to run your program as the only application. Want to support and promote mostly static binaries where tasks can be defined at run time and can make maximum use of distributed resources, but don't want to bother supporting dynamic processes and resource usage. Also want to make it hard to represent states that are memory unsafe and prone to data races by basing the OS in rust and promoting a static, build and compile focused method to creating an application using the OS. Also possible to just do multitasking, but idk its extra work for something linux/mac/win do much more effciently.

# Todo

[ ] x86_64 kernel not including preemptive multitasking 
[ ] risc-v fork

# Usage

Configure the repository to use the nightly rust compiler:

```
rustup override set nightly
```

Install QEMU, this can be done using your package manager:

```
sudo apt install qemu-system
```

Run the WIP OS in a vm:

```
cargo run
```
