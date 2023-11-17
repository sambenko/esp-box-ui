#![no_std]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, text::Text, Drawable,
    primitives::{RoundedRectangle, PrimitiveStyleBuilder, Rectangle, Primitive},
    image::Image, 
    prelude::{DrawTarget, Dimensions, Point, RgbColor, WebColors, Size},
};

use core::fmt::Write as FmtWrite;
use profont::PROFONT_18_POINT;
use tinybmp::Bmp;

// Constants for sensor icon BMP data
const TEMPERATURE_ICON: &[u8] = include_bytes!("../icons/temperature.bmp");
const PRESSURE_ICON: &[u8] = include_bytes!("../icons/pressure.bmp");
const HUMIDITY_ICON: &[u8] = include_bytes!("../icons/humidity.bmp");

pub enum SensorType {
    Temperature,
    Pressure,
    Humidity,
}

pub struct SensorData {
    pub sensor_type: SensorType,
    pub pos_x: i32,
    pub value: f32,
}

const FIELD_WIDTH: u32 = 70;
const ICON_OFFSET: i32 = 3;
const TEXT_OFFSET: i32 = 23;
const POS_Y: i32 = 70;

pub fn draw_sensor_data<D>(display: &mut D, sensor_data: &SensorData)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    draw_icon(display, sensor_data);
    draw_field(display, sensor_data);
}

fn draw_icon<D>(display: &mut D, sensor_data: &SensorData)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let icon_data = match sensor_data.sensor_type {
        SensorType::Temperature => TEMPERATURE_ICON,
        SensorType::Pressure => PRESSURE_ICON,
        SensorType::Humidity => HUMIDITY_ICON,
    };

    let icon = Bmp::from_slice(icon_data).unwrap();
    Image::new(&icon, Point::new(sensor_data.pos_x + ICON_OFFSET, POS_Y)).draw(display);
}

fn draw_field<D>(display: &mut D, sensor_data: &SensorData)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(5)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::CSS_ALICE_BLUE)
        .build();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(sensor_data.pos_x, POS_Y + FIELD_WIDTH as i32), Size::new(FIELD_WIDTH, 35)),
        Size::new(10, 10),
    )
    .into_styled(style)
    .draw(display);

    update_sensor_data(display, sensor_data);
}

pub fn update_sensor_data<D>(display: &mut D, sensor_data: &SensorData)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);
    let text_position = Point::new(sensor_data.pos_x + 15, POS_Y + FIELD_WIDTH as i32 + TEXT_OFFSET);

    let mut data_string: heapless::String<16> = heapless::String::new();
    write!(data_string, "{:.1}", sensor_data.value).unwrap();

    Text::new(&data_string, text_position, text_style).draw(display);
}
