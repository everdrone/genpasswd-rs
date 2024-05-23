# Genpasswd

Strong password generator in your terminal.

## Install

```shell
git clone https://github.com/everdrone/genpasswd-rs.git
cargo install --path ./genpasswd-rs
```

## Usage

```
Usage: genpasswd [OPTIONS]

Options:
  -n, --sequences <NUM>   The number of alphanumeric sequences [default: 3]
  -l, --length <NUM>      The length of each sequence [default: 6]
  -d, --digits <NUM>      The total number of digits [default: 1]
  -u, --uppercase <NUM>   The total number of uppercase letters [default: 1]
  -s, --separator <CHAR>  The separator character [default: -]
  -h, --help              Print help
  -V, --version           Print version
```

Examples:

```shell
genpasswd
# Output: st5lqn-kiyohk-hmXwzo

genpasswd -n 2 -l 9 -d 3 -u 3 -s _
# Output: qehlUwwli_4Naek29gz

genpasswd -n1 -l12 -d2 -u10
# Output: 49RXVGLFHIEX
```