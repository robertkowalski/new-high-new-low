[![Build Status](https://travis-ci.org/robertkowalski/new-high-new-low.svg?branch=master)](https://travis-ci.org/robertkowalski/new-high-new-low)

# nhnl

Gets the New-High / New-Low data for different stock markets.

## Compile & install

```
cargo build --release

# executable appears as:
./target/release/nhnl

# install
mv ./target/release/nhnl /usr/local/bin/

```

## Usage

```
$ nhnl --help

Usage: nhnl [--help | --json]

--json         print result as json
--help         print help
```
