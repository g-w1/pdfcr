use crate::{HEIGHT, WIDTH};
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

    title_layer.use_text("TITLE", 50, Mm(WIDTH / 2.0), Mm(HEIGHT / 2.0), &font);
    (doc, font)
}
