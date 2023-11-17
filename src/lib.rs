#![no_std]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    text::Text,
    Drawable,
};
use profont::PROFONT_18_POINT;

use core::fmt::Write as FmtWrite;

use tinybmp::Bmp;

pub mod sensor_data;
use sensor_data::{draw_sensor_data, SensorData};

fn sensor_overlay<D>(display: &mut D)
where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
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

pub fn build_sensor_ui<D>(
    display: &mut D,
    temperature_data: &SensorData,
    humidity_data: &SensorData,
    pressure_data: &SensorData,
) where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    sensor_overlay(display);
    draw_sensor_data(display, &temperature_data);
    draw_sensor_data(display, &humidity_data);
    draw_sensor_data(display, &pressure_data);
}

pub mod food_item;
use food_item::{build_food_item, FoodItem};

pub fn build_inventory<D>(
    display: &mut D,
    hotdog: &FoodItem,
    sandwich: &FoodItem,
    energy_drink: &FoodItem,
) where
    D: DrawTarget<Color = Rgb565> + Dimensions,
{
    // Build the UI for each food item
    build_food_item(display, &hotdog);
    build_food_item(display, &sandwich);
    build_food_item(display, &energy_drink);
}
