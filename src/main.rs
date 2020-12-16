use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use util::*;
use walkdir::WalkDir;
mod util;

const WIDTH: f64 = 200.0;
const HEIGHT: f64 = 264.0;
const MAX_HEIGHT_TEXT: usize = 48;

const HELP_TXT: &'static str = "pdfcr version 1.0
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
";

fn main() {
    let opts = parse_cli();
    let (mut doc, font) = init_doc(opts.title.as_str(), opts.title.as_str());

    for input in opts.inputs {
        for e in WalkDir::new(input) {
            match e {
                Ok(x) => {
                    if x.path().is_dir() {
                        continue;
                    }
                    let path = x.path().to_str().unwrap();
                    let c = match CodeFile::from_file(path, font.clone()) {
                        Ok(z) => z,
                        Err(e) => {
                            if !opts.abort_on_binary {
                                eprintln!(
                                    "Could not render file '{}' because {}, skipping it",
                                    path, e
                                );
                                continue;
                            } else {
                                eprintln!(
                                    "Could not render file '{}' because {}, aborting",
                                    path, e
                                );
                                exit();
                            }
                        }
                    };
                    c.print_page(&mut doc);
                    println!("Rendered: {}", path);
                    drop(c);
                }
                Err(e) => {
                    eprintln!("Could not render: {}", e);
                    exit();
                }
            }
        }
    }

    println!("saving document...");
    match doc.save(&mut BufWriter::new(match File::create(&opts.output_file) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("could not write file: {}", e);
            exit();
        }
    })) {
        Ok(_) => {
            println!("Saved into: {}", opts.output_file);
        }
        Err(e) => {
            eprintln!("could not save doc: {}", e);
            exit();
        }
    }
}

struct CodeFile {
    text: String,
    name: String,
    font: IndirectFontRef,
}

impl CodeFile {
    pub fn print_page(&self, doc: &mut PdfDocumentReference) {
        let font_size = 11;
        let spacing = font_size as f64 / 2.1;

        let (page, mut layer) = add_new_page(doc, &self.name);
        doc.add_bookmark(self.name.clone(), page.page);

        let mut i = 0;
        let mut line_num_ctr = 0;

        for line in self.text.lines() {
            if i >= MAX_HEIGHT_TEXT {
                layer = add_new_page(doc, &self.name).1;
                i = 0;
            }
            let mut b = true;
            for bruh in textwrap::wrap(line, 85).iter() {
                i += 1;
                let mut _line: String;
                if b {
                    _line = line_num_ctr.to_string();
                    _line.push(' ');
                    b = false;
                } else {
                    _line = String::new();
                }
                _line.push_str(bruh);
                layer.use_text(
                    _line,
                    font_size as f64,
                    Mm(2.0),
                    Mm(264.0 - spacing * i as f64 - spacing),
                    &self.font,
                );
            }
            line_num_ctr += 1;
        }
    }
    fn from_file(fname: &str, font: IndirectFontRef) -> Result<Self, Error> {
        let text = std::fs::read_to_string(fname)?;
        Ok(Self {
            text,
            name: fname.to_string(),
            font,
        })
    }
}
