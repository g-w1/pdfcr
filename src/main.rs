use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use util::*;
mod util;

const WIDTH: f64 = 200.0;
const HEIGHT: f64 = 264.0;
const MAX_HEIGHT_TEXT: usize = 48;

fn main() -> Result<(), Error> {
    let (mut doc, font) = init_doc("TITLE", "CODE");

    // let c = CodeFile::from_file("ir.cpp", font)?;
    // //////////////////////////////////////////////////////////////////////////////// //////////////////////////////////////////////////////////////////////////////// //////////////////////////////////////////////////////////////////////////////// //////////////////////////////////////////////////////////////////////////////// ////////////////////////////////////////////////////////////////////////////////
    let c = CodeFile::from_file("src/main.rs", font)?;
    c.print_page(&mut doc);

    doc.save(&mut BufWriter::new(File::create("code.pdf").unwrap()))
        .unwrap();
    Ok(())
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

        let (page,mut layer)  = add_new_page(doc, &self.name);
        doc.add_bookmark("BRUH", page.page);

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
