#![no_std]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, prelude::*, text::Text, Drawable,
    primitives::{ RoundedRectangle, PrimitiveStyleBuilder, Rectangle },
};
use profont::PROFONT_18_POINT;

use core::fmt::Write as FmtWrite;

use tinybmp::Bmp;

pub mod temperature;
use temperature::{ temperature_icon, temperature_field };

pub mod humidity;
use humidity::{ humidity_icon, humidity_field };

pub mod pressure;
use pressure::{ pressure_icon, pressure_field };

fn overlay<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::BLACK)
            .fill_color(Rgb565::WHITE)
            .build();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(19, 20), Size::new(280, 200)),
            Size::new(10, 10),
        )
        .into_styled(style)
        .draw(display);
}

pub fn build_ui<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        overlay(display);

        temperature_icon(display);
        temperature_field(display);

        humidity_icon(display);
        humidity_field(display);

        pressure_icon(display);
        pressure_field(display);
}

pub mod hotdog;
use hotdog::{ hotdog_icon, hotdog_field, hotdog_overlay, buy_one_button };

pub mod sandwich;
use sandwich::{ sandwich_icon, sandwich_field };

pub mod energy_drink;
use energy_drink::{ energy_drink_icon, energy_drink_field };

pub fn build_inventory<D>(display: &mut D)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {

        hotdog_overlay(display);
        hotdog_icon(display);
        hotdog_field(display, 0, 0.0);
        buy_one_button(display);

        sandwich_icon(display);
        sandwich_field(display, 0, 0.0);

        energy_drink_icon(display);
        energy_drink_field(display, 0, 0.0);
}