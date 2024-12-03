use crate::{HEIGHT, HELP_TXT, WIDTH};
use printpdf::*;

pub fn add_new_page(
    doc: &mut PdfDocumentReference,
    fname: &str,
) -> (PdfPageReference, PdfLayerReference) {
    let (page, layer) = doc.add_page(Mm(WIDTH), Mm(HEIGHT), fname);
    (doc.get_page(page), doc.get_page(page).get_layer(layer))
}

pub fn init_doc(title: &str, layer_name: &str) -> (PdfDocumentReference, IndirectFontRef) {
    let (mut doc, title_page, title_layer) =
        PdfDocument::new(title, Mm(WIDTH), Mm(HEIGHT), layer_name);
    doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
        ..Default::default()
    }));
    let title_layer = doc.get_page(title_page).get_layer(title_layer);

    let mut font_reader =
        std::io::Cursor::new(include_bytes!("../assets/JetBrainsMono-Regular.ttf").as_ref());
    let font = doc.add_external_font(&mut font_reader).unwrap();

    title_layer.use_text(title, 50.0, Mm(0.0), Mm(HEIGHT / 2.0), &font);
    doc.add_bookmark("title page", title_page);
    (doc, font)
}
pub fn exit() -> ! {
    std::process::exit(1)
}

pub fn parse_cli() -> CliOpts {
    use std::env::args;
    let mut cmd_args_tmp = args().collect::<Vec<String>>();
    cmd_args_tmp.remove(0);
    let mut inputs = Vec::new();
    let mut output_file = None;
    let mut abort_on_binary = false;
    let mut it = cmd_args_tmp.iter();
    let mut opt_t: Option<String> = None;
    let mut include_line_numbers = true;
    while let Some(i) = it.next() {
        match i.as_str() {
            "-o" => {
                output_file = match it.next() {
                    Some(x) => Some(x.clone()),
                    None => {
                        eprintln!("expected an output file after \"-o\"\n{} ", HELP_TXT);
                        exit();
                    }
                }
            }
            "--title" | "-t" => {
                opt_t = match it.next() {
                    Some(x) => Some(x.clone()),
                    None => {
                        eprintln!("expected an argument after \"--title\"\n{} ", HELP_TXT);
                        exit();
                    }
                }
            }
            "--stop-on-bad-file" | "-s" => abort_on_binary = true,
            "--no-line-numbers" | "-n" => include_line_numbers = false,
            n => {
                if n.starts_with("-") {
                    eprintln!("unexpected option: {}\n{}", n, HELP_TXT);
                    exit();
                }
                inputs.push(n.to_string());
            }
        }
    }
    if inputs.len() < 1 {
        eprintln!("");
        eprintln!("{}", HELP_TXT);
        exit();
    }
    if output_file.is_none() {
        eprintln!("printpdf needs one output file");
        eprintln!("");
        eprintln!("{}", HELP_TXT);
        exit();
    }

    let title = match opt_t {
        Some(t) => t,
        None => "TITLE".to_string(),
    };
    CliOpts {
        inputs,
        output_file: output_file.unwrap(),
        title,
        abort_on_binary,
        include_line_numbers,
    }
}

pub struct CliOpts {
    pub inputs: Vec<String>,
    pub output_file: String,
    pub title: String,
    pub abort_on_binary: bool,
    pub include_line_numbers: bool,
}
