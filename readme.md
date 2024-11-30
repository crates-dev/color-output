# COLOR-OUTPUT

[Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/)

## Description

- An atomic output library based on Rust that supports output functionalities through functions, builders, and other methods. It allows customization of text and background colors.

## Features

- Supports formatted output of the current time.
- Allows customization of text color, background color, font weight, and other configurations.
- Supports defining structures for output messages.
- Supports defining builders for output messages.
- Supports single-line output for multiple tasks.
- Supports multi-line output for multiple tasks.
- Ensures atomic output operations.

## Installation

To install `ltpp-output` run cmd:

```shell
cargo add ltpp-output
```

## Code Examples

### Struct Output

#### Using `output` Function

```rust
use color_output::*;
output(Output {
    text: "test_output_struct",
    text_color: ColorType::Use(Color::Default),
    text_bg_color: ColorType::Color256(0x000000),
    show_time: true,
    time_text_color: ColorType::Rgb(255, 255, 255),
    time_bg_color: ColorType::Use(Color::Yellow),
    split: " => ",
    split_color: ColorType::Use(Color::Cyan),
    split_bg_color: ColorType::Use(Color::Yellow),
    endl: true,
    ..Default::default()
});
```

#### Using `output` Method

```rust
use color_output::*;
Output {
    text: "test_output_struct_output",
    text_color: ColorType::Use(Color::Default),
    text_bg_color: ColorType::Use(Color::Blue),
    show_time: true,
    time_text_color: ColorType::Rgb(255, 255, 255),
    time_bg_color: ColorType::Use(Color::Yellow),
    split: " => ",
    split_color: ColorType::Use(Color::Cyan),
    split_bg_color: ColorType::Use(Color::Yellow),
    endl: true,
    ..Default::default()
}
.output();
```

### Array of Structs

```rust
use color_output::*;
OutputList(vec![
    Output {
        text: "test_output_list_struct_1",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Color256(0x000000),
        show_time: true,
        time_text_color: ColorType::Rgb(255, 255, 255),
        time_bg_color: ColorType::Use(Color::Yellow),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: false,
        ..Default::default()
    },
    Output {
        text: "test_output_struct_output_2",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Use(Color::Blue),
        show_time: true,
        time_text_color: ColorType::Rgb(255, 255, 255),
        time_bg_color: ColorType::Use(Color::Yellow),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    },
])
.output();
```

### Builder Output

#### Using `output` Function

```rust
use color_output::*;
output(
    OutputBuilder::new_from(Output::default())
        .text("test_output_builder")
        .text_color(ColorType::Color256(0xffffff))
        .text_bg_color(ColorType::Color256(0xffffff))
        .split_bg_color(ColorType::Color256(0xffffff))
        .time_text_color(ColorType::Rgb(255, 200, 255))
        .text_blod(true)
        .time_text_blod(true)
        .show_time(true)
        .endl(true)
        .build(),
);
```

#### Using `output` Method

```rust
use color_output::*;
OutputBuilder::new()
    .text("test_output_builder_output")
    .text_bg_color(ColorType::Color256(0xffffff))
    .text_color(ColorType::Color256(0xffffff))
    .time_text_color(ColorType::Rgb(255, 200, 255))
    .text_blod(true)
    .time_text_blod(true)
    .show_time(true)
    .endl(true)
    .build()
    .output();
```

### Array Builder

```rust
use color_output::*;
OutputListBuilder::new_from(vec![Output::default()])
    .add(
        OutputBuilder::new()
            .text("text")
            .text_bg_color(ColorType::Use(Color::Blue))
            .endl(false)
            .build(),
    )
    .add(Output {
        text: "test_new_from_output_list_builder_1",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Color256(0x3f3f3f),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: false,
        ..Default::default()
    })
    .add(Output {
        text: "test_new_from_output_list_builder_2",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Use(Color::Cyan),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    })
    .run();
```

### Output Macros

#### Passing Struct

```rust
use color_output::*;
output_macro!(Output {
    text: "test_proc_macro",
    text_color: ColorType::default(),
    text_bg_color: ColorType::Use(Color::Yellow),
    show_time: true,
    time_text_color: ColorType::Use(Color::Green),
    time_bg_color: ColorType::Color256(0xffffff),
    split: " => ",
    split_color: ColorType::Use(Color::Cyan),
    split_bg_color: ColorType::Use(Color::Yellow),
    endl: true,
    ..Default::default()
});
```

#### Passing Builder

```rust
use color_output::*;
output_macro!(OutputBuilder::new()
    .text("test_output_builder")
    .text_color(ColorType::Use(Color::Cyan))
    .time_text_color(ColorType::Use(Color::Blue))
    .text_blod(true)
    .time_text_blod(true)
    .show_time(true)
    .endl(true)
    .build());
```

#### Multiple Inputs

```rust
use color_output::*;
output_macro!(
    Output {
        text: "test_proc_macro",
        text_color: ColorType::default(),
        text_bg_color: ColorType::Use(Color::Yellow),
        show_time: true,
        time_text_color: ColorType::Use(Color::Green),
        time_bg_color: ColorType::Color256(0xffffff),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    },
    OutputBuilder::new()
        .text("test_output_builder1")
        .text_color(ColorType::Color256(0xffffff))
        .time_text_color(ColorType::Rgb(255, 200, 255))
        .text_blod(true)
        .time_text_blod(true)
        .show_time(true)
        .endl(true)
        .build(),
    OutputBuilder::new()
        .text("test_output_builder2")
        .text_color(ColorType::Color256(0xffffff))
        .time_text_color(ColorType::Rgb(255, 200, 255))
        .text_blod(true)
        .time_text_blod(true)
        .show_time(true)
        .endl(true)
        .build()
);
```

### Color Usage

- `ColorType::Use`: Use built-in colors.
- `ColorType::Color256`: Hexadecimal colors.
- `ColorType::Rgb`: RGB color (r, g, b).

#### ColorType::Use

```rust
ColorType::Use(Color::White)
```

#### ColorType::Color256

```rust
ColorType::Color256(0xffffff)
```

#### ColorType::Rgb

```rust
ColorType::Rgb(255, 255, 255)
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
