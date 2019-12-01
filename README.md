# launchpad-pro-rs

A fork of [dvhrd's launchpad-pro](https://github.com/dvhdr/launchpad-pro) project to enable writing open source firmware for the Launchpad Pro in Rust.

# Setup

## macOS
First you'll need Rust installed which you can get at [rustup.rs](https://rustup.rs/). You will also need [Homebrew](https://brew.sh/).

We need to install the cross compilation target for the Launchpad Pro:
```
$ rustup target add thumbv7m-none-eabi
```

The following commands will install [cargo-binutils](https://github.com/rust-embedded/cargo-binutils). This lets us transform our Cargo output (ELF) into hex format.
```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

We also need to install [cargo-make](https://github.com/sagiegurari/cargo-make). This lets us define the tasks and flow for creating the SysEx file that we can upload to the Launchpad Pro.
```
$ cargo install cargo-make
```

Finally we need to install the GCC ARM toolchain using Homebrew.
```
$ brew install armmbed/formulae/arm-none-eabi-gcc
```

# Building

## Local

By default Cargo will build your project for the host machine. This means we can use an `std` environment for running our tests.

We can build our project by running:

```
$ cargo build
```

And as expected we can test our project by running: 

```
$ cargo test
```

## Launchpad Pro

You will need to build your project as a SysEx file in order to upload it to the Launchpad Pro. To do this run:

```console
$ cargo sysex --bin main
```

This will create the firmware image, `app.syx`, in the `build` directory. This can then be uploaded to the Launchpad Pro. Consult the [guide from the original repository](https://github.com/dvhdr/launchpad-pro#uploading-to-a-launchpad-pro) on how to do this.

# Examples

## Game of Life

A simple implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life). To build it for the Launchpad run:

```
$ cargo sysex --example life
```

Pressing buttons on the grid will create new life! You can pause the simulation by pressing the Setup button.