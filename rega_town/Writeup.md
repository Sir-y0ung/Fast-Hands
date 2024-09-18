
Prepared By: `YoungFlexer`

Challenge Author(s): `YoungFlexer`

Difficulty: <font color='orange'>Medium</font>

<br><br>

***NOTE : The headings with `(!)` should be necessarily included in your writeup while the ones with `(*)` are optional and should be included only if there is a need to. Of course, you can modify the content of each section accordingly. We just provide some boilerplate text.***

# Synopsis (!)

- User must parse and understand some complex regex to transform them into z3 constants !

## Description (!)

- Welcome to Rega Town, a quaint little place where everyone communicates through the magic of patterns and rules!

## Skills Required (!)

- Rust
- Researching Skills
- Regex
- Know how to use common RE tools (i.e. Ghidra, IDA)
- z3-solver

## Skills Learned (!)

- Learn how regex rules work.
- Learn how to debug executables.
- Learn how to solve linear systems of equations.

# Enumeration (!)
## Running the binary
we prompt for a secret passphrase hmm


## Analyzing the source code (*)
Lets start with the file command!
```
rega_town: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, BuildID[sha1]=5223aaede99eb8790c66284d76ddaebe527e70a5, for GNU/Linux 3.2.0, with debug_info, not stripped
```
Ok so we have to deal with a 64 bit not stripped rust binary!\n
running strings on we can detect some regex patterns that seems interesting like these
```
(?:.[^0-9]*\d){5}
.{24}\x54.\x65.\x54.*
```

### Ida part
Dissasembpling our executable we came accross to 3 different functions except main
`
filter_input
check_input
multiply_characters
`

# Solution (!)
