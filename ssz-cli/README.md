# ssz-cli

This command line tool is light wrapper around Parity's [SSZ library](https://crates.io/crates/ssz)
which deserializes `JSON` using [ssz-json](https://github.com/c-o-l-o-r/ssz-json) and encode it in
SSZ. This works especially well for prototyping Ethereum 2.0 execution environments in 
[Scout](https://github.com/ewasm/scout).

## Usage
Pass a [properly formatted](https://github.com/c-o-l-o-r/ssz-json#format) `JSON` file to the cli and
let it convert it to its ssz equivalent.
```
USAGE:
    ssz [FLAGS] <filename>

FLAGS:
    -b               display result in a byte array
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## Maintainer
* [@matt_garnett](https://twitter.com/matt_garnett)

## License
Apache 2.0
