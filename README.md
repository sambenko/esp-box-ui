# ESP-Box UI Crate

Welcome to the ESP-Box UI crate! This crate is engineered to draw elegant UI elements on any ESP-Box devices, providing a visual representation of sensor data measured using a BME680 sensor connected to the device. The UI elements include readings for humidity, pressure, and temperature. 

## Features

- **Drawing UI Elements**: Utilize a suite of functions to draw and update UI elements representing sensor data on your display.
- **Embedded Graphics**: Leverages the [Embedded Graphics](https://github.com/embedded-graphics/embedded-graphics) crate for drawing shapes.
- **ProFont Integration**: Utilizes the [ProFont](https://github.com/wezm/profont) crate for rendering text, displaying measured data in a clear, readable font.
- **TinyBMP for Icons**: Incorporates the [TinyBMP](https://github.com/embedded-graphics/tinybmp) crate for displaying BMP images as icons next to the sensor data.

## Contents

This crate contains three main Rust files, each dedicated to a specific type of sensor data: `humidity.rs`, `pressure.rs`, and `temperature.rs`. Each file contains functions to draw an icon representing the data type, a field to display the data, and a function to update the displayed data.

## Usage

To use this crate in your project, ensure you have the following entry in your `Cargo.toml` file:

```toml
[dependencies]
esp-box-ui = { git = "https://github.com/sambenko/esp-box-ui.git" }
```
Then you can just build the ui with all elements like this:

```rust
use esp_box_ui::build_ui;

// Your display initialisation code...

build_ui(&mut display);

// Rest of your code...
```


