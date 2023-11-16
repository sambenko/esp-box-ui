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
const POS_Y: i32 = 170;

pub fn energy_drink_icon<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let icon_data = include_bytes!("../icons/humidity.bmp");
    let energy_drink = Bmp::from_slice(icon_data).unwrap();
    Image::new(&energy_drink, Point::new(POS_X, POS_Y)).draw(display);
}

pub fn energy_drink_field<D>(display: &mut D, amount: i32, price: f32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(3)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::CSS_ALICE_BLUE)
        .build();

    // Placeholder for the amount
    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + 60, POS_Y), Size::new(50, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    // Placeholder for the price
    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + 120, POS_Y), Size::new(50, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    update_energy_drink(display, amount, price);
}

pub fn update_energy_drink<D>(display: &mut D, amount: i32, price: f32)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    let amount_position = Point::new(POS_X + 65, POS_Y + 5);
    let price_position = Point::new(POS_X + 125, POS_Y + 5);

    let mut amount_string: heapless::String<16> = heapless::String::new();
    let mut price_string: heapless::String<16> = heapless::String::new();

    write!(amount_string, "{}", amount).unwrap();
    write!(price_string, "${:.2}", price).unwrap();

    Text::new(&amount_string, amount_position, text_style).draw(display);
    Text::new(&price_string, price_position, text_style).draw(display);
}
