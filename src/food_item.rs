#![no_std]
#![allow(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyle, pixelcolor::Rgb565, text::Text, Drawable,
    primitives::{RoundedRectangle, PrimitiveStyleBuilder, Rectangle, Primitive },
    image::Image, 
    prelude::{DrawTarget, Dimensions, Point, RgbColor, WebColors, Size},
};

use core::fmt::Write as FmtWrite;
use profont::PROFONT_18_POINT;
use tinybmp::Bmp;

pub struct FoodItem<'a> {
    pub name: &'a str,
    pub pos_y: i32,
    pub amount: i32,
    pub price: f32,
}

const HOTDOG_ICON: &[u8] = include_bytes!("../icons/hot-dog.bmp");
const SANDWICH_ICON: &[u8] = include_bytes!("../icons/sandwich.bmp");
const ENERGY_DRINK_ICON: &[u8] = include_bytes!("../icons/energy_drink.bmp");


const POS_X: i32 = 10;
const FIELD_WIDTH_AMOUNT: i32 = 50;
const FIELD_WIDTH_PRICE: i32 = 70;
const LABEL_OFFSET: i32 = 80;
const BOTTOM_PADDING: i32 = 10;
const ITEM_HEIGHT: u32 = 65;

pub fn build_food_item<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    draw_border(display, food_item);
    draw_icon(display, food_item);
    draw_field(display, food_item);
    draw_buy_button(display, food_item);
}

fn draw_icon<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let icon_data = match food_item.name {
        "Hotdog" => HOTDOG_ICON,
        "Sandwich" => SANDWICH_ICON,
        "Energy Drink" => ENERGY_DRINK_ICON,
        _ => return,
    };

    let icon = Bmp::from_slice(icon_data).unwrap();
    Image::new(&icon, Point::new(POS_X + 10, food_item.pos_y + 10)).draw(display);
}

fn draw_field<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(3)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::CSS_ALICE_BLUE)
        .build();

    let label_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    Text::new("Amount", Point::new(POS_X + LABEL_OFFSET - 10, food_item.pos_y + 18), label_style)
        .draw(display);

    Text::new("Price", Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + 25, food_item.pos_y + 18), label_style)
        .draw(display);

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + LABEL_OFFSET, food_item.pos_y + 25), Size::new(FIELD_WIDTH_AMOUNT as u32, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + 20, food_item.pos_y + 25), Size::new(FIELD_WIDTH_PRICE as u32, 30)),
        Size::new(5, 5),
    )
    .into_styled(style)
    .draw(display);

    update_field(display, food_item);
}

fn update_field<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let text_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    let amount_position = Point::new(POS_X + LABEL_OFFSET + 17, food_item.pos_y + 45);
    let price_position = Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + 25, food_item.pos_y + 45);

    let mut amount_string: heapless::String<16> = heapless::String::new();
    let mut price_string: heapless::String<16> = heapless::String::new();

    write!(amount_string, "{}", food_item.amount).unwrap();
    write!(price_string, "${:.2}", food_item.price).unwrap();

    Text::new(&amount_string, amount_position, text_style).draw(display);
    Text::new(&price_string, price_position, text_style).draw(display);
}

fn draw_border<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let border_style = PrimitiveStyleBuilder::new()
        .stroke_width(5)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::WHITE)
        .build();

        RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::new(POS_X, food_item.pos_y), Size::new(300, ITEM_HEIGHT)),
            Size::new(10, 10),
        )
        .into_styled(border_style)
        .draw(display);
}

fn draw_buy_button<D>(display: &mut D, food_item: &FoodItem)
where 
    D:DrawTarget<Color = Rgb565>+Dimensions {
    let button_style = PrimitiveStyleBuilder::new()
        .stroke_width(3)
        .stroke_color(Rgb565::BLACK)
        .fill_color(Rgb565::CSS_ALICE_BLUE)
        .build();

    let label_style = MonoTextStyle::new(&PROFONT_18_POINT, RgbColor::BLACK);

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + FIELD_WIDTH_PRICE + 30, food_item.pos_y + 10), Size::new((FIELD_WIDTH_PRICE - 10) as u32, 45)),
        Size::new(5, 5),
    )
    .into_styled(button_style)
    .draw(display);
    
    Text::new("BUY", Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + FIELD_WIDTH_PRICE + 42, food_item.pos_y + 30), label_style)
        .draw(display);
    
    Text::new("1", Point::new(POS_X + LABEL_OFFSET + FIELD_WIDTH_AMOUNT + FIELD_WIDTH_PRICE + 55, food_item.pos_y + 48), label_style)
        .draw(display);
}
