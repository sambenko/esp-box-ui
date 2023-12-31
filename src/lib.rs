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
        Rectangle::new(Point::new(19, 50), Size::new(280, 150)),
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
    if hotdog.highlighted {
        build_food_item(display, &hotdog, Rgb565::CSS_DARK_GREEN, Rgb565::CSS_DARK_SEA_GREEN);
    } else {
        build_food_item(display, &hotdog, Rgb565::CSS_BLACK, Rgb565::CSS_WHITE);
    }

    if sandwich.highlighted {
        build_food_item(display, &sandwich, Rgb565::CSS_DARK_GREEN, Rgb565::CSS_DARK_SEA_GREEN);
    } else {
        build_food_item(display, &sandwich, Rgb565::CSS_BLACK, Rgb565::CSS_WHITE);
    }

    if energy_drink.highlighted {
        build_food_item(display, &energy_drink, Rgb565::CSS_DARK_GREEN, Rgb565::CSS_DARK_SEA_GREEN);
    } else {
        build_food_item(display, &energy_drink, Rgb565::CSS_BLACK, Rgb565::CSS_WHITE);
    }
}
