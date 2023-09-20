# Agape Deus

This is a simple CLI tool to fetch and print today's daily readings in the catholic church, written in Rust

# Usage

```bash
$ agape --help

Usage: agape [OPTIONS]

Options:
  -s, --sunday   Get the readings for the upcoming sunday mass
  -h, --help     Print help information
  -V, --version  Print version information
  
```

# Source

The readings are retrieved from [Universalis.com](https://universalis.com/)

# Installation

## Linux

### Install Rust (if necessary)

You will need to have the Rust build utility ``cargo`` installed. 
As this does not appear to be distributed on its own, you will need
to install Rust.

How you do this will depend on your distribution. For example,
on Manjaro, you would do as follows:

```
    $ sudo pamac install rust
```

### Clone This repo

This assumes you have the ``git`` client installed. 

```
    $ git clone https://github.com/ngorden/agape-deus.git
```


### Build The Development Version

Assuming you've just cloned this repo into the current directory:

```
    $ cd agape-deus
    $ cargo build
```


* You should see output from cargo indicating download of dependencies and a
successful build. 

* You should now have the debug version of the executable in 
``./target/debug/agape``

### Test the Executable

```
    ./target/debug/agape
    ./agape
```

This should print today's readings to the terminal.
Run ``agape --help`` for further information. 

### Make the Executable Available Globally

* You will need to have ``readlink`` installed (it is typically installed as a
standard utility in most Linux distributions).

* The precise target location for the symbolic link may vary depending
on your distribution. Typically, it should be ``/usr/bin``.

```
    sudo ln -s $(echo $(readlink -f ./target/debug/agape)) /usr/bin/agape
```

You should now be able to invoke ``agape`` from any directory in a terminal
session. 
