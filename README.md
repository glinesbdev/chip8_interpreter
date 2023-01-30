# CHIP-8 Interpreter

A basic CHIP-8 interpreter that can run roms without any extensions i.e. Super CHIP-8.

No roms? No problem! Upon running this emulator, you will be presented with roms from [chip8Archive][archive] -- a repo full of CHIP-8 roms all licenced under [Creative Commons 0][cc0]. This list will only contain roms that can be run by this interpreter.

## Input Mapping
| Original | Remapped |
| -------- | -------- |
| <table><tr><td>1</td><td>2</td><td>3</td><td>C</td></tr><tr><td>4</td><td>5</td><td>6</td><td>D</td></tr><tr><td>7</td><td>8</td><td>9</td><td>E</td></tr><tr><td>A</td><td>0</td><td>B</td><td>F</td></tr></table> | <table><tr><td>1</td><td>2</td><td>3</td><td>4</td></tr><tr><td>Q</td><td>W</td><td>E</td><td>R</td></tr><tr><td>A</td><td>S</td><td>D</td><td>F</td></tr><tr><td>Z</td><td>X</td><td>C</td><td>V</td></tr></table> |

# Playing!

> Because the SDL2 C library is dynamically linked via the `sdl2` crate, you will need to build this project. If the SDL2 dynamic library isn't automatically generated, you can get it from here: [SDL2 Releases][sdl2lib].

Clone this repo and run `cargo build --release` in the project's root directory. Once built, you can run the executable in the `target/release` folder. Be sure to verify that the SDL2 library is in the same directory as the executable or it will not work!

# Resources

* [mattmikolay's CHIP-8 Technical References][mattmikolay]
* [Cowgod's Techinical Referece][cowgod]
* [Overview on how emulators work][emulators]

# Main dependencies
 * [sdl2][sdl2]
 * [imgui][imgui]
 * [glow][glow]
 * [serde][serde]
 * [serde_json][json]
 * [reqwest][reqwest]

[archive]: https://github.com/JohnEarnest/chip8Archive
[cc0]: https://creativecommons.org/share-your-work/public-domain/cc0
[mattmikolay]: https://github.com/mattmikolay/chip-8
[cowgod]: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
[emulators]: https://multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter
[sdl2lib]: https://github.com/libsdl-org/SDL/releases/tag/release-2.26.2
[sdl2]: https://docs.rs/sdl2/latest/sdl2
[imgui]: https://docs.rs/imgui/latest/imgui
[glow]: https://docs.rs/glow/latest/glow
[serde]: https://docs.rs/serde/latest/serde
[json]: https://docs.rs/serde_json/latest/serde_json
[reqwest]: https://docs.rs/reqwest/latest/reqwest
