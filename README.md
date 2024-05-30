# Plant Watering Sensor

This project is a fork of [arm-hello-world-rust](https://github.com/caiotavares/arm-hello-world-rust).
Use the **Setup instructions** below taken from [arm-hello-world-rust](https://github.com/caiotavares/arm-hello-world-rust) to initialize this project.

This repository aims to read a [Soil Moisture Sensor Hygrometer Module](https://www.amazon.de/-/en/Aideepen-Hygrometer-Capacitive-Corrosion-Resistant/dp/B08GCRZVSR/ref=sr_1_1?crid=2W7XF8F4JKXZQ&dib=eyJ2IjoiMSJ9.Ny3kivPWKCRgx8Sjtrz7Fw.ohgDE1o5dPTlRXRLrAiAA_s1_pvnoX7SqPY2ZAuswf0&dib_tag=se&keywords=Aideepen+6+Pieces+Soil+Moisture+Sensor+Hygrometer+Module+V1.2+Capacitive+Corrosion+Resistant+Module+for+Plant+Watering+DIY&qid=1717084973&s=industrial&sprefix=aideepen+6+pieces+soil+moisture+sensor+hygrometer+module+v1.2+capacitive+corrosion+resistant+module+for+plant+watering+diy+%2Cindustrial%2C101&sr=1-1) to detect whether your plants need to be watered.

Check the [Feature List](feature_list.md) for features which are being worked on or to add your own requests.



# Setup Instructions

ARM Cortex M4F application built using the rust-embedded [template](https://github.com/rust-embedded/cortex-m-quickstart)

## Target

This program is targeted at MSP432P401R Launchpad development board

## Getting Started

Install dependencies:

- [OpenOCD](http://openocd.org/)
- [GNU ARM Embedded Toolchain](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm)

Make sure to install the latest commit of OpenOCD, which includes the MSP432 board configuration.

On macOS:
```bash
$ brew install openocd --HEAD
```
You may also need to install `openocd` dependencies: `autoconf`, `automake` and `texinfo`

On other platforms, follow the instructions on https://sourceforge.net/p/openocd/code/ci/master/tree/ in the section *Compiling OpenOCD*

## Running

Setup Rust beta and add the ARM build platform target:
```bash
$ rustup default beta
$ rustup target add thumbv7em-none-eabihf
```

Clone and compile the project:
```bash
$ git clone git@github.com:caiotavares/arm-hello-world-rust.git
$ cd arm-hello-world-rust
$ cargo build
```

Open a OpenOCD server and leave it open in a terminal:

```bash
$ openocd
```

On a separate terminal, open the GDB client:

```bash
$ arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/arm-hello-world-rust
$ (gdb) target remote :3333
$ (gdb) load
$ (gdb) monitor arm semihosting enable
$ (gdb) continue
```

You should see `Hello, world!` displayed at the OpenOCD terminal
