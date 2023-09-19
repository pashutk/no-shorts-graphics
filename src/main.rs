use embedded_graphics::{
    mono_font::ascii::FONT_9X18_BOLD,
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb888,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(135, 240));

    let bounding_box = display.bounding_box();

    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(Rgb888::CSS_DEEP_PINK)
        .build();

    let center_aligned = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Justified)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    #[derive(Debug)]
    enum TypeCPin {
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        A11,
        A12,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        B11,
        B12,
    }

    let groups = vec![
        vec![TypeCPin::A1, TypeCPin::A2],
        vec![TypeCPin::A3, TypeCPin::A4, TypeCPin::A5, TypeCPin::A6],
        vec![
            TypeCPin::B4,
            TypeCPin::B5,
            TypeCPin::B6,
            TypeCPin::B7,
            TypeCPin::B8,
            TypeCPin::B9,
            TypeCPin::B10,
            TypeCPin::B11,
        ],
    ];

    let groups: Vec<String> = groups
        .into_iter()
        .map(|group| group.into_iter().map(|pin| format!("{:?}", pin)).collect())
        .map(|group: Vec<String>| {
            let a: String = group.join(", ");
            a
        })
        .collect();

    let result = groups.join("\n");

    TextBox::with_textbox_style(
        result.as_str(),
        bounding_box.offset(-8),
        character_style,
        center_aligned,
    )
    .draw(&mut display)?;

    Window::new("No shorts", &OutputSettings::default()).show_static(&display);

    Ok(())
}
