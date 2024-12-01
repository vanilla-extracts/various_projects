# Various projects

projects done in Rust to learn the language.

TODO List

- [x] cat
- [x] ls
- [x] calc (done under its own [project](https://github.com/vanilla-extracts/calc))

# Building

To build the projects do

```bash
cargo build --release
```

or just 

```bash
make release
```

## Cat

Don't let the name fool you, it is _not_ a full reimplementation of the POSIX
_cat_, it just reads the content of a file

```bash 
./target/release/cat <file>
```

### Usage

![cat](assets/cat.png)

## LS

LS: a _very_ minimalistic *bare bone* implementation of ls

```bash 
./target/release/ls <dir>
```

It _just_ list files in a dir.

### Usage

You can just do

```bash 
./target/release/ls <dir>
```

which produces

![](assets/ls_vanilla.png)

However this one has _options_

### Options

#### Colour

You can add _colours_ to the output with the `--color` or `-c` option
![](assets/ls_colors.png)

#### Sort

You can sort alphabetically the output with the `--sort` or `-s` option
![](assets/ls_sort.png)

#### Both

And you can combine them to have a _beautiful_ **sorted** output
![](assets/ls_colors_sort.png)
