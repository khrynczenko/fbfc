# fbfc (find banned function calls)

## What is `fbfc`

`fbfc` is a small command line tool that searches C/C++ source code files
for the *banned function calls* according to the the [Security Development
Lifecycle (SDL) Banned Function Calls](
https://docs.microsoft.com/en-us/previous-versions/bb288454(v=msdn.10)) article
which itself is derived from the *The Security Development Lifecycle* book by
Michael Howard and Steve Lipner.

## How to use `fbfc`

You can pass a path to a directory where your C/C++ source code files are as
an argument.

```bash
$ fbfc cpp/

Category:      MemoryCopy
Function name: memcpy
File:          cpp/source.cpp
Line #:        2
Line:              memcpy(nullptr, 5);
Possible fix:  Use memcpy_s or wmemcpy_s function.

Category:      StringConcat
Function name: strcatA
File:          cpp/source.cpp
Line #:        3
Line:              strcatA();
Possible fix:  Use String*Cat or String*CatEx or strcat_s function.

Category:      Scanf
Function name: scanf
File:          cpp/source.cpp
Line #:        4
Line:              scanf();
Possible fix:  Use sscanf_s function.

Found banned functions calls: 3
```

Fore more information:  

```bash
$ fbfc --help
...
```

## How to build `fbfc`

`fbfc` is built using cargo. Download the source code and type `cargo build`
in your terminal.
