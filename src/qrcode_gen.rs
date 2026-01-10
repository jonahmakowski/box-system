use super::database;
use ::image::ImageEncoder;
use ::image::Luma;
use ::image::codecs::png::PngEncoder;
use printpdf::*;
use qrcode::QrCode;
use std::fs::File;
use std::io::Write;

fn gen_qr_code(data: &str) -> QrCode {
    QrCode::new(data.as_bytes()).unwrap()
}

pub fn gen_qr_code_for_box(data: &database::BoxData) -> QrCode {
    gen_qr_code(data.uuid())
}

pub fn to_pdf(code: QrCode, save_to: &str) {
    let mut doc = PdfDocument::new("QR Code");
    let image = code.render::<Luma<u8>>().build();

    // Encode the image as PNG
    let mut png_data = Vec::new();
    let encoder = PngEncoder::new(&mut png_data);
    encoder
        .write_image(
            image.as_raw(),
            image.width(),
            image.height(),
            ::image::ColorType::L8.into(), // Convert ColorType to ExtendedColorType
        )
        .unwrap();

    let mut warnings = Vec::new();
    let image = RawImage::decode_from_bytes(&png_data, &mut warnings).unwrap();

    // In the PDF, an image is an `XObject`, identified by a unique `ImageId`
    let image_xobject_id = doc.add_image(&image);

    let page1_contents = vec![Op::UseXobject {
        id: image_xobject_id.clone(),
        transform: XObjectTransform::default(),
    }];

    // Retrieve the width and height of the image from its fields
    let image_width = image.width as f64;
    let image_height = image.height as f64;

    let mm_per_pixel = 25.4 / 300.0;
    let page_width = Mm((image_width * mm_per_pixel) as f32);
    let page_height = Mm((image_height * mm_per_pixel) as f32);

    // Create the PDF page with the dimensions of the image
    let page1 = PdfPage::new(page_width, page_height, page1_contents);
    let pdf_bytes: Vec<u8> = doc
        .with_pages(vec![page1])
        .save(&PdfSaveOptions::default(), &mut warnings);

    let mut file = File::create(save_to).unwrap();
    file.write_all(&pdf_bytes).unwrap();
}
