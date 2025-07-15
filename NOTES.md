# Brief Phrases

## In Rust, if your program compiles, it will probably run.

## You can't segfault if you don't have null.

## Rust does not hide complexity from developers, it offers them the right tools to manage all the complexity.

# Why Rust? Isn't Node.js good enough?

## 1. Type Safety
Rust's strong type system enforces many errors at compile time, reducing runtime bugs and improving code reliability.

## 2. Systems Language Features
Rust provides low-level control over system resources such as RAM and CPU, which is essential for performance-critical applications. 
Meaning Rust is a low-level language that runs very close to the hardware, allowing for fine-grained control over system resources.
Compared to JavaScript, Rust is incredibly fast and efficient.
The code is statically analyzed, which means that many errors can be caught at compile time before building and optimising the binary for performance. This binary then runs directly on the CPU, making it much faster than interpreted languages like JavaScript.


## 3. Concurrency
Running multiple tasks on multiple cores is called concurrency. For example, if we run a code in Node.js, it will run on a single thread from let's say 32 cores.
Whereas in Rust, we can run multiple tasks on multiple cores, which is called parallelism.
Rust's ownership model and concurrency features allow for safe and efficient parallel execution, making it easier to

## 4. Memory Safety
Here's where C and Rust differ. C does not have a memory safety model, which means that it is possible to write code that can cause memory leaks or segmentation faults.

Rust has a concept of owners,borrowing and lifetimes, which ensures that memory is managed safely and efficiently.



# Initializing a Rust Project
- run `cargo init` to create a new Rust project in pwd

It created 2 files:
- `Cargo.toml`: This is the manifest file for Rust's package manager, Cargo. It contains metadata about the project, such as its name, version, dependencies, and other configuration options.(similar to package.json in Node.js)

- `src/main.rs`: This is the main source file for the Rust project. It contains the entry point of the program, which is the `main` function. This is where the execution of the program starts.


## `cargo init` 

1. We can create an end user application with `cargo init --bin` or a library with `cargo init --lib`.

Simply running `cargo init` will create a binary project by default, kinda like a end user application backend.


`cargo build` will compile the project and create an executable in the `target/debug` directory. (For development builds)

`cargo build --release` will compile the project and create an optimized executable in the `target/release` directory. (For production builds)

if we run `target/debug/hello` it will run the executable.

# Simple Variables

### Numbers

```rust
i8 // 8-bit signed integer
i16 // 16-bit signed integer
i32 // 32-bit signed integer
i64 // 64-bit signed integer
i128 // 128-bit signed integer

u8 // 8-bit unsigned integer
u16 // 16-bit unsigned integer
u32 // 32-bit unsigned integer
u64 // 64-bit unsigned integer
u128 // 128-bit unsigned integer

f8 // 8-bit floating point (not supported in Rust)
f16 // 16-bit floating point
f32 // 32-bit floating point
f64 // 64-bit floating point
f128 // 128-bit floating point (not supported in Rust)


```

For instance, i8 can store 2^7 values, which is from -128 to 127. why? because first bit is used for the sign (positive or negative), leaving 7 bits for the value. Whereas u8 can store 2^8 values, which is from 0 to 255, because it does not need to store the sign, leaving all 8 bits for the value.

By default, Rust uses `i32` for signed integers and `u32` for unsigned integers.
By default, Rust uses `f64` for floating point numbers.

------------------------------------------------------------------------

### Signed vs Unsigned
- Signed integers can represent both positive and negative numbers.
- Unsigned integers can only represent positive numbers, but they can represent a larger range of positive numbers than signed integers of the same size.


-------------------------------------------------------------------------

In Rust, variables are immutable by default, meaning that once a variable is assigned a value, it cannot be changed. To make a variable mutable, we need to use the `mut` keyword.

```rust
let mut x = 5;
x = 10;
```


-------------------------------------------------------------------------

## Booleans
```rust
let is_active: bool = true;
let is_logged_in: bool = false;
```

-------------------------------------------------------------------------

## Strings
We can't just do
```rust
let name = "John";
```
Because strings are not fixed data type and can cause RAM issues, they keep on changing in size, so we need to use the `String` type, which is a growable, heap-allocated data structure, it is the most common way to represent strings in Rust.

```rust
let name: String = String::from("John");
```

