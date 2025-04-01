# Rust_calculator

A calculator application made in rust with use of slint library for gui

## About
This project created on slint example template for learing rust and slint gui library.
Project created just for learning and fun. Warning it has bugs, shit code and no logic.

## Usage

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`.
2. Download and extract the [ZIP archive of this repository](https://github.com/n0rdye/rust_calculator).
3. Rename the extracted directory and change into it:
    ```
    mv rust_calculator project
    cd project    
    ```
4. Build with `cargo`:
    ```
    cargo build
    ```
5. Run the application binary:
    ```
    cargo run
    ```

## Dependencies

async-std-1.13.1;
regex-1.11.1;
slint-1.8.0;

### For building

slint-build-1.8.0

## Sate of the project

Currently there's no number checks except devision by zero.
This project just for my learning of rust language so it has no big future.
The code might be not safe, working, acceptable or correct following of rust guidelines.

## Realeses

Get them here (https://github.com/n0rdye/rust_calculator/releases)


## Bugs/todo

need to remake calculation function - all callculations as in closures;

if value isn't inserted so if prints zero in array its error so error - num results in zero;