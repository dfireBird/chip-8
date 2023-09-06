# Chip-Octo
Chip-Octo is interpreter of the classic [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) programming language made in Rust.

The interpreter's implementation targets the implementation of the CHIP-8 made for the [COSMAC VIP](https://en.wikipedia.org/wiki/COSMAC_VIP) machine.
It's may not have the quirks expected from later implementations.

The implementation is tested using [Timendus's](https://github.com/Timendus/) [chip-8 test suite](https://github.com/Timendus/chip8-test-suite). It passed all of the tests expect the quirk test.

## Usage
To start the installed chip-octo with ROM file path as the only argument:
```
$ chip-octo <rom_file_path>
```

To see all options available to the user:
```
$ chip-octo --help
```

## Configuration 
Chip-Octo looks for configuration file in the default configuration directory as given by [dirs](https://crates.io/crates/dirs) crate.
See [here](https://docs.rs/dirs/5.0.1/dirs/fn.config_dir.html), for the configuration directory in various platforms.

If not found there, it looks for configuration file in the same directory, the executable was invoked in.
Even then, if it's not found it's uses the default configuration.

CLI Arguments takes precedence over configuration in the config file. Please be mindful when using both. 

See the [example configuration file](chip-octo.toml) to see the available config options.

## Keybindings
The implementation follows the original COSMAC VIP keypad.
By default, the keys in the table below respectively maps the keypad of the COSMAC VIP.
The keymap can be changed using the config file, see the [example config file](chip-octo.toml) for example.

Keys:
| 1 | 2 | 3 | 4 |
|---|---|---|---|
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | v |

Image Reference of keypad:

<img src="https://tobiasvl.github.io/assets/images/cosmac-vip-keypad.png" alt="COSMAC VIP keypad" width="200" />

## Installation
Chip-Octo is not published anywhere. But you can install directly from source using [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
```
$ cargo install --git github.com/dfireBird/chip-octo
```

### Building

If you wanna build from source yourself:

1.  Clone the source
```
$ git clone https://github.com/dfireBird/chip-octo
```
2.  Use cargo to build with release flag
```
$ cargo build --flag
```
3.  You can install it using cargo, which will copy the binary to `~/.cargo/bin/` (assuming your prefix is `~/.cargo`)
```
$ cargo install
```
## TODO
- [x] Configuration system.
- [x] Add Documentation regarding config and command line parameters
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

Chip-Octo is primarily distributed under the terms of MIT License.
See [LICENSE](LICENSE) for details.
