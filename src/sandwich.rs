#![no_std]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, text::Text, Drawable,
    primitives::{ RoundedRectangle, PrimitiveStyleBuilder, Rectangle, Primitive },
    image::Image, 
    prelude::{DrawTarget, Dimensions, Point, RgbColor, WebColors, Size},
};

use core::fmt::Write as FmtWrite;
use profont::PROFONT_18_POINT;
use tinybmp::Bmp;

const POS_X: i32 = 10;
const POS_Y: i32 = 90;
const FIELD_WIDTH: i32 = 100;
const LABEL_OFFSET: i32 = 60;

pub fn sandwich_icon<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let icon_data = include_bytes!("../icons/sandwich.bmp");
    let sandwich = Bmp::from_slice(icon_data).unwrap();
    Image::new(&sandwich, Point::new(POS_X, POS_Y)).draw(display);
}

pub fn sandwich_field<D>(display: &mut D, amount: i32, price: f32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(3)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::CSS_ALICE_BLUE)
        .build();

    let label_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    // Draw "Amount" label
    Text::new("Amount:", Point::new(POS_X + LABEL_OFFSET + 10, POS_Y + 10), label_style)
        .draw(display);

    // Draw "Price" label
    Text::new("Price:", Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH + 20, POS_Y + 10), label_style)
        .draw(display);

    // Draw amount field
    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + LABEL_OFFSET + 10, POS_Y + 20), Size::new(FIELD_WIDTH as u32, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    // Draw price field
    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH + 20, POS_Y + 20), Size::new(FIELD_WIDTH as u32, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    update_sandwich(display, amount, price);
}

pub fn update_sandwich<D>(display: &mut D, amount: i32, price: f32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    let amount_position = Point::new(POS_X + LABEL_OFFSET + 15, POS_Y + 30);
    let price_position = Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH + 25, POS_Y + 30);

    let mut amount_string: heapless::String<16> = heapless::String::new();
    let mut price_string: heapless::String<16> = heapless::String::new();

    write!(amount_string, "{}", amount).unwrap();
    write!(price_string, "${:.2}", price).unwrap();

    Text::new(&amount_string, amount_position, text_style).draw(display);
    Text::new(&price_string, price_position, text_style).draw(display);
}