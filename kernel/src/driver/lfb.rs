use embedded_graphics::{
    image::Image, prelude::Point, prelude::*, primitives::Circle, primitives::PrimitiveStyle,
};
use tinybmp::Bmp;

pub fn init() {
    let fb = crate::limine::framebuffer0();

    let bmp_data = include_bytes!("../../../cat.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();
    Image::new(&bmp, Point::new(200, 200)).draw(fb).unwrap();
}
