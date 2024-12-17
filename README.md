# Makefile Generator
CLI tool written in Rust to generate makefiles for compiling programs.

This tool is ***unfinished*** and can currently only create a makefile for .c & .h files 

Current Goals:
* Allow for multiple targets
* Refactor code & learn about error handling with files in Rust
* Learn about the command path on macOS (allow for accessibility anywhere)
* Allow for other languages to be used (c++, asm, rust, etc.)

## Setup
### Project Folder
Open your computer's terminal and cd to a directory of your choice.

Ensure [git](https://git-scm.com/) is installed on your system, then clone this repository using:

```sh
git clone https://github.com/Cortes205/MakefileGenerator
```

or you can download it manually as a zipfile.

## Usage
### Execute
Open the terminal in the program's directory

Ensure you have the following:
* [rustc](https://www.rust-lang.org/tools/install) (Rust compiler)

To compile this program, use the following command:

```sh
rustc src/main.rs -o makegen 
```

Now you can move this tool to any of your project directories and use it!

For the help menu (when it's created) use the command:
```sh
./makegen -h
```

Otherwise, to make use of the tool, use the command:
```sh
./makegen -c <target filename> <.c files> <.h files> -<flags>
```

The order of arguments only matters for the first two.

* -c indicates you are going to be compiling a C program (only option for now)
* The target file must be the second argument
* .c files, .h files, and compiler flags can be ordered in any way, but they follow a few rules:

1. At least one .c file must be provided
2. Non .c or .h files will cause the program to exit
3. Any compiler flags must have a preceding '-' (much like actual compilation)
4. Using the flag "-default" will use the flags: "-std=c99 -Wall -pedantic"

Compilation flags will not be checked by the program, but you will evidently receive an error when trying to run the makefile!

### Example
An example of running this program in any of your project directories:
```sh
./makegen -c bin/program src/*.c -default include/*.h -lm
```
The ordering of the last four arguments is odd, but the tool is robust
and creates a correct makefile!

### Runtime
There is nothing to be input during runtime! 

The only thing that happens is a file called makefile will be outputted and will be ready for use.

## About This Project
This project was a way for me to learn the Rust programming langauge while solving a personal problem of mine. The majority of my programming classes in school are done in C and I hate writing makefiles for the 10+ source files I have to create. I mean, who doesn't? Yes, there are online tools that already generate makefiles, but there's no learning in that, so they're not fun! With this mini-project, I am learning yet another embedded systems language, how to code in vim, more about git, what makes a good CLI tool, and the command path on Mac. The learning never stops!

[LinkedIn](https://www.linkedin.com/in/cortes205/)
