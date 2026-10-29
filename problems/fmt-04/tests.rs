use crate::judge::{case, check_eq};
use crate::user::Color;

fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

pub fn run() {
    case("example-1", || {
        let red = color(255, 0, 16, 255);
        let glass = color(255, 0, 16, 128);
        check_eq(format!("{red:?}"), "Rgb(255, 0, 16)")?;
        check_eq(format!("{red:x?}"), "Rgb(ff, 0, 10)")?;
        check_eq(format!("{glass:?}"), "Rgba(255, 0, 16, 128)")?;
        check_eq(format!("{red:#X?}"), "Rgb(\n    0xFF,\n    0x0,\n    0x10,\n)")
    });
    case("case-1", || {
        // Any alpha other than 255 gives `Rgba`, including 254 and 0.
        check_eq(format!("{:?}", color(1, 2, 3, 254)), "Rgba(1, 2, 3, 254)")?;
        check_eq(format!("{:X?}", color(171, 205, 239, 0)), "Rgba(AB, CD, EF, 0)")?;
        check_eq(format!("{:X?}", color(171, 205, 239, 255)), "Rgb(AB, CD, EF)")
    });
    case("case-2", || {
        // Inside other values: one more level of indentation, and the flags still apply.
        check_eq(
            format!("{:#?}", [color(0, 0, 0, 0)]),
            concat!(
                "[\n",
                "    Rgba(\n",
                "        0,\n",
                "        0,\n",
                "        0,\n",
                "        0,\n",
                "    ),\n",
                "]",
            ),
        )?;
        check_eq(format!("{:x?}", Some(color(10, 20, 30, 255))), "Some(Rgb(a, 14, 1e))")
    });
}
