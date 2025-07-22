# Description

Idea is to have a kernel centered around parallel programming in rust using partitioned address spaces. No preemptive multitasking, no processes, no paged memory. But does have multithreading and abstractions for distributed programming by creating a partitioned global address space abstraction across multiple machines that have the OS. Idea is you either just link against the library and boot/run from a container, nanovm or vm, OR you link against a mini-kernel image and boot into the OS itself, putting your program in the main process loop. Also maaaaybe possible to do a form of multitasking using partitioned addresses rather than paged memory as an abstraction, but again linux, windows and mac already exist, so why bother making a multifunctional monolithic kernel.

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
