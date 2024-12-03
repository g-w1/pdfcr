# pdfcr

Install: `cargo install pdfcr`.

# PDF CODE RENDERER:

This is a utility to take a files and turn it into a pdf optimised for reading on a kindle (or anywhere) with bookmarks.

```
pdfcr version 1.0
usage:
pdfcr [files]... [directories]... [--stop-on-bad-file | -s] [--title | -t TITLE] -o output-file.pdf

file: an optional list of files to render
directories: an optional list of directories to render
NOTE: at least one file or directory must be provided

--stop-on-bad-file | -s: if pdfcr finds a file such as a binary file, it will not skip it (default), but stop and not render an output file

--title | -t: specify the title of the document, default is TITLE

-o: the output pdf file to render to, required

examples:

pdfcr src -o code.pdf # classic example
pdfcr src Cargo.toml -o code.pdf -t \"is this a quine?\" # this renders the src directory and a Cargo.toml file to code.pdf, with a title of \"is this a quine?\"
pdfcr cmd -o test.pdf --stop-on-bad-file # renders every file in cmd to test.pdf, but if it encounters binary files, it aborts the rendering
```

An example rendered file is [example_out.pdf](./example_out.pdf) from this codebase using this command `pdfcr src/ Cargo.toml .gitignore LICENSE README.md -o example_out.pdf -t "Is this quine?"`.
The font used is [JetBrains Mono](https://www.jetbrains.com/lp/mono/), which is embedded into the binary.

This has much higher speeds, and a lower memory footprint than the main competitor, [render50](https://github.com/cs50/render50). The reason that I made this was that I wanted to view a very large codebase on a kindle, and render50 used over 4gb of ram to render it, which was unacceptable.

```bash
❯ time render50 src -o out.pdf
Rendered src/main.rs.
Rendered src/util.rs.
Rendered out.pdf.

real	0m27.082s
user	0m8.432s
sys	0m0.743s
❯ time pdfcr src -o out.1.pdf
Rendered: src/util.rs
Rendered: src/main.rs
saving document...
Saved into: out.1.pdf

real	0m0.125s
user	0m0.113s
sys	0m0.012s
```
