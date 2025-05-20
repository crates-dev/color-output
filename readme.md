<center>

# color-output

[![](https://img.shields.io/crates/v/color-output.svg)](https://crates.io/crates/color-output)
[![](https://img.shields.io/crates/d/color-output.svg)](https://img.shields.io/crates/d/color-output.svg)
[![](https://docs.rs/color-output/badge.svg)](https://docs.rs/color-output)
[![](https://github.com/eastspire/color-output/workflows/Rust/badge.svg)](https://github.com/eastspire/color-output/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/color-output.svg)](./LICENSE)

</center>

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

To install `color-output` run cmd:

```shell
cargo add color-output
```

## Code Examples

### Struct Output

#### Using `output` Function

```rust
use color_output::*;
output(Output {
    text: "test_output_struct",
    color: ColorType::Use(Color::Default),
    bg_color: ColorType::Color256(0x000000),
    endl: true,
    ..Default::default()
});
```

#### Using `output` Method

```rust
use color_output::*;
Output {
    text: "test_output_struct_output",
    color: ColorType::Use(Color::Default),
    bg_color: ColorType::Use(Color::Blue),
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
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Color256(0x000000),
        endl: false,
        ..Default::default()
    },
    Output {
        text: "test_output_struct_output_2",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Use(Color::Blue),
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
        .color(ColorType::Color256(0xffffff))
        .bg_color(ColorType::Color256(0xffffff))
        .blod(true)
        .endl(true)
        .build(),
);
```

#### Using `output` Method

```rust
use color_output::*;
OutputBuilder::new()
    .text("test_output_builder_output")
    .bg_color(ColorType::Color256(0xffffff))
    .color(ColorType::Color256(0xffffff))
    .blod(true)
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
            .bg_color(ColorType::Use(Color::Blue))
            .endl(false)
            .build(),
    )
    .add(Output {
        text: "test_new_from_output_list_builder_1",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Color256(0x3f3f3f),
        endl: false,
        ..Default::default()
    })
    .add(Output {
        text: "test_new_from_output_list_builder_2",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Use(Color::Cyan),
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
    color: ColorType::default(),
    bg_color: ColorType::Use(Color::Yellow),
    endl: true,
    ..Default::default()
});
```

#### Passing Builder

```rust
use color_output::*;
output_macro!(OutputBuilder::new()
    .text("test_output_builder")
    .color(ColorType::Use(Color::Cyan))
    .blod(true)
    .endl(true)
    .build());
```

#### Multiple Inputs

```rust
use color_output::*;
output_macro!(
    Output {
        text: "test_proc_macro",
        color: ColorType::default(),
        bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    },
    OutputBuilder::new()
        .text("test_output_builder1")
        .color(ColorType::Color256(0xffffff))
        .blod(true)
        .endl(true)
        .build(),
    OutputBuilder::new()
        .text("test_output_builder2")
        .color(ColorType::Color256(0xffffff))
        .blod(true)
        .endl(true)
        .build()
);
```

#### print_success!

> Outputs success information without a new line.

```rust
use color_output::*;
print_success!("1234", "5678");
```

#### print_warning!

> Outputs warning information without a new line.

```rust
use color_output::*;
print_warning!("1234", "5678");
```

#### print_error!

> Outputs error information without a new line.

```rust
use color_output::*;
print_error!("1234", "5678");
```

#### println_success!

> Outputs success information with a new line.

```rust
use color_output::*;
println_success!("1234", "5678");
```

#### println_warning!

> Outputs warning information with a new line.

```rust
use color_output::*;
println_warning!("1234", "5678");
```

#### println_error!

> Outputs error information with a new line.

```rust
use color_output::*;
println_error!("1234", "5678");
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

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
