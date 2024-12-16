use embedded_graphics::{
    image::Image, prelude::Point, prelude::*, primitives::Circle, primitives::PrimitiveStyle,
};
use tinybmp::Bmp;

pub fn init() {
    let fb = crate::limine::framebuffer0();
    let circle = Circle::new(Point::new(22, 22), 140).into_styled(PrimitiveStyle::with_stroke(
        crate::limine::LColor {
            r: 250,
            g: 250,
            b: 250,
        },
        1,
    ));
    circle.draw(fb).unwrap();

    let bmp_data = include_bytes!("../../../cat.bmp");
    let bmp = Bmp::from_slice(bmp_data).unwrap();
    Image::new(&bmp, Point::new(200, 200)).draw(fb).unwrap();

    /*
    //draw_pixel(fb, 600, 600, (250, 250, 250)).unwrap();
    let mut a = 0;
    loop {
        for x in 1..1200 {
            for y in 1..800 {
                draw_pixel(fb, x, y, (a, 250 - a, a)).unwrap();
            }
        }
    }*/
}
