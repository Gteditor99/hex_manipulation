Hex_Manipulation
=========================================================

Introduction::
------------

Hex Manipulation like the one used in the `cicadia 3301` puzzle on 4chan.

What is Hex Manipulation?::
--------------------------

Hex manipulation is the process of converting data from one format to another using hexadecimal 'notation'.
hexadecimal representation is not obsfucation, but data can be hidden in a file (typically at the end of the file) and then extracted using hex manipulation.

--------------------------

To build this project on windows, you will need to install the following::

* [Rust](https://www.rust-lang.org/tools/install)
* And it's dependencies.

run the following command in the root directory of the project:
    
    1) `cargo build --release`
    
    The resulting binary will be located in `target/release/`


Or:
    get the latest windows-x64 build from https://github.com/Gteditor99/hex_manipulation/releases/latest
--------------------------
Usage::

    1) `hex_manipulation.exe` to run the binary.
    2) Drag and drop a file onto your terminal to open said file.
    3) Once the file is done loading, you will be prompted to enter an input.
    4) Press enter to exit the program, feel free to check the output file yourself to see if it worked.
--------------------------
