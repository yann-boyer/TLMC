TLMC is a small and basic Chip8 Emulator written in Rust with Rust-SDL2, made for fun.


If you are on a Linux based system, run these following commands to build the project :



First, you will need SDL2 and SDL2_Mixer :



On Arch :


```
$ pacman -Sy sdl2 sdl2_mixer
```


On Fedora :


```
$ sudo dnf install SDL2 SDL2-devel SLD2_mixer SDL2_mixer-devel
```


On Debian/Ubuntu :


```
$ sudo apt-get install libsdl2 libsdl2-dev libsdl2-mixer libsdl2-mixer-dev
```


Now, you can build the project :


```
$ git clone https://github.com/yann-boyer/TLMC.git
$ cd TLMC
$ cargo build --release
```


Now, you can run TLMC :


```
$ cp chip8_beep.mp3 target/release/
$ cd target/release
$ ./TLMC /path/to/the/rom
```
(TLMC on upper case or lower case !)


Copyright (c) 2022 - Yann BOYER.
