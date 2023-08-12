# Chip-8
Chip8 is interpreter of the classic [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) programming language made in Rust.

The interpreter's implementation targets the implementation of the CHIP-8 made for the [COSMAC VIP](https://en.wikipedia.org/wiki/COSMAC_VIP) machine.
It's may not have the quirks expected from later implementations.

The implementation is tested using [Timendus's](https://github.com/Timendus/) [chip-8 test suite](https://github.com/Timendus/chip8-test-suite). It passed all of the tests expect the quirk test.

## Installation
Chip8 is not published anywhere. But you can install directly from source using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
```
$ cargo install --git github.com/dfireBird/chip-8
```

### Building

If you wanna build from source yourself:

1.  Clone the source
```
$ git clone https://github.com/dfireBird/chip-8
```
2.  Use cargo to build with release flag
```
$ cargo build --flag
```
3.  You can install it using cargo, which will copy the binary to `~/.cargo/bin/` (assuming your prefix is `~/.cargo`)
```
$ cargo install
```

## Usage
To start the installed chip8 with ROM file path as the only argument:
```
$ chip8 <rom_file_path>
```
or if you are working on source and wanna launch with Cargo:
```
$ cargo run -- <rom_file_path>
```

## Keybindings

The implementation follows the original COSMAC VIP keypad.
The keys in table below respectively maps the keypad of the COSMAC VIP.

Keys:
| 1 | 2 | 3 | 4 |
|---|---|---|---|
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | v |

Image Reference of keypad:

<img src="https://tobiasvl.github.io/assets/images/cosmac-vip-keypad.png" alt="COSMAC VIP keypad" width="200" />

## TODO
- [ ] Configuration system.
- [ ] Build a GUI to manage settings and ROM files.

## Credits and References

-   [Public Domain CHIP-8 Games](https://www.zophar.net/pdroms/chip8.html)
-   [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
-   [Awesome CHIP-8](https://chip-8.github.io/links/)
-   [Instruction Set Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set)

## Contributing

Create an issue if you have any suggestions or you find any issues with implementation.
If you want to make the change yourself, please create an issue before creating the PR itself.

## License

Chip8 is primarily distributed under the terms of MIT License.
See [LICENSE](LICENSE) for details.
