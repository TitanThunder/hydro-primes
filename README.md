# Hydro Template

This repository contains example programs demonstrating the use of Hydro, a framework for building distributed systems in Rust.

## Setup

In general, you can follow the [official instructions](https://www.rust-lang.org/tools/install) to install Rust. Every instruction you find in this document you will also find there.

### Windows
Select your installer on [rustup.rs](https://rustup.rs/), download and run it, and follow the on-screen instructions.

### MacOS & Linux
You should be good to go after running:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
When prompted, choose the __default installation__. After installation, restart your terminal.

### Last steps
To see if the installation worked, you can run:
```bash
rustc --version
```
If you get an error, you might need to add the `bin` directory of cargo, Rust's package manager, to your __`$PATH` environment__.


You might run into errors when trying to compile something for the first time. These are most likely to be related to a __missing or falsely configured C/C++ compiler-toolchain__. In that case, look up possible solutions to that error online.

We also recommend restarting your computer after installation to ensure everything works correctly.

## Running the code

We’ve collected a few simple examples to help you get started with Hydro. They gradually increase in computational demand and complexity, showcasing what Hydro is capable of.
Hydro is especially performant in parallelizing compute-heavy processes, even only on local hardware, without using networks or other more complex concepts of the distributed systems field.

### Running with Hydro
This example consists of the [deployment file](examples/cluster.rs) and the [process file](src/cluster.rs). 
It shows parallelization in action by allocating strings to the processes and letting them perform operations on these strings (see `line 14` of [cluster](src/cluster.rs)). 
Furthermore, especially the ordering capabilities of Hydroflow can be seen here, if you compare the output order of the individual workers with that of the leader process (this might need a few tries to work 100%).

```bash
cargo run --example cluster
```
In `line 14` of [examples/cluster](examples/cluster.rs), you can see something like this: 
`.with_cluster(&workers, vec![deployment.Localhost(); 8])`.
You can replace the number at the end of the line to change the number of workers, i.e. processes running in parallel.
You should also see a difference in the order of the output.

### Generate Primes
This example demonstrates how Hydro can be used to simulate a real-world scientific computation: generating primes using the Sieve of Eratosthenes. 
It includes a basic sequential implementation, a standard parallelized one using the built-in Rust threads functionality, and a parallelized version using Hydro.

#### Sequentially
This version of the program, called [src/sequential](src/sequential.rs), runs on a single thread and generates primes using the Sieve of Eratosthenes. It serves as a baseline for understanding the logic before introducing parallelization.
```bash
cargo run --example sequential
```

#### In Parallel (built-in Rust)
This version parallelizes the prime number generation using Rust’s built-in multithreading support via the `std::thread` module. 
The core logic of the Sieve of Eratosthenes is split into chunks, each handled by a separate thread to take advantage of multiple CPU cores.
```bash
cargo run --example parallel_rust
```
The implementation, found in [examples/parallel_std](examples/parallel_std.rs), demonstrates how you can divide the sieve range among threads, compute primes in each subrange, and then merge the results. 
This significantly reduces computation time on multi-core systems compared to the sequential version.

While more stable than the Hydro version, this approach introduces some complexity in managing thread synchronization and result aggregation.

#### In Parallel (Hydro)
This example, also consisting of the [deployment file](examples/parallel_hydro.rs) and the [process file](src/parallel_hydro.rs), finally visualizes a real-world use case for Hydro by [generating primes using the Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
```bash
cargo run --example parallel_hydro
```
Just like above, in [examples/parallel_hydro](examples/parallel_hydro.rs), you can replace the number at the end of `line 14` to change the number of workers. Increasing it should speed up the calculations.

Unfortunately, the program is not particularly reliable, so you may have to restart it for it to work properly, the output may freeze, or the program may crash completely without notice.

### Stopping the code
Due to conceptual reasons, Hydro-programs run until you manually stop them, even if they are not calculating anything anymore. 
To terminate them, you can either press `Ctrl + C` or manually kill the terminal by closing it. 
If you don't do this, the processes running in parallel might fill up your memory leading to performance issues.
