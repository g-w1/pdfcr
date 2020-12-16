# PDFCR

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
