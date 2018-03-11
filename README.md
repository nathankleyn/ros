# ROS

ROS (Rust Operating System) is a barebones kernel written in Rust whilst following [Phil Opperman's "Writing an OS in Rust" series][Phil OS Blog].

## Requirements

* Rust Nightly: Install via Rustup for ease-of-use.
* Xargo: Used for cross-compiling Rust sysroots. Install using `cargo install bootimage`.
* Bootimage: Used to wrap the resulting kernel image in a bootloader. Install using `cargo install bootimage`.
* QEMU: Used for booting the kernel image and testing it.

## Usage

Compilation of the kernel only can be done by compiling for the one and only supported target (x86_64-ros, for which this repository includes [the target specification](/x86_64-ros.json)):

```sh
xargo build --target x86_64-ros
```

The resulting kernel needs to be wrapped in a bootloader, which is the job of `bootimage`:

```sh
bootimage --target x86_64-ros
```

In fact, calling `bootimage` will automatically run the above `xargo` command, so you can just run this command if what you want is the bootloader wrapped kernel between changes.

Running `bootimage` produces a `bootimage.bin` file in the root of the repository, which is the bootable disk image itself. You can run this now using QEMU:

```sh
qemu-system-x86_64 -drive format=raw,file=bootimage.bin
```

It is also possible to write it to an USB stick and boot it on a real machine:

```sh
dd if=bootimage.bin of=/dev/sdX && sync
```

Where `sdX` is the device name of your USB stick. Be careful to choose the correct device name, because everything on that device is overwritten.

[Phil OS Blog]: https://os.phil-opp.com/second-edition/
