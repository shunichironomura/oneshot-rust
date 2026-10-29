use std::collections::BTreeMap;

use crate::judge::{case, check_eq};
use crate::user::{Color, Credentials, Inventory};

fn creds(user: &str, password: &str, scopes: &[&str]) -> Credentials {
    Credentials {
        user: user.to_string(),
        password: password.to_string(),
        scopes: scopes.iter().map(|s| s.to_string()).collect(),
    }
}

fn inventory(items: &[(&str, u32)], cache: &[u8]) -> Inventory {
    let mut map = BTreeMap::new();
    for &(name, count) in items {
        map.insert(name.to_string(), count);
    }
    Inventory { items: map, cache: cache.to_vec() }
}

fn color(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color { r, g, b, a }
}

pub fn run() {
    case("example-1", || {
        let c = creds("ann", "hunter2", &["read", "write"]);
        check_eq(
            format!("{c:?}"),
            r#"Credentials { user: "ann", password: <redacted>, scopes: ["read", "write"] }"#,
        )?;
        check_eq(
            format!("{c:#?}"),
            concat!(
                "Credentials {\n",
                "    user: \"ann\",\n",
                "    password: <redacted>,\n",
                "    scopes: [\n",
                "        \"read\",\n",
                "        \"write\",\n",
                "    ],\n",
                "}",
            ),
        )
    });
    case("example-2", || {
        let inv = inventory(&[("pear", 1), ("apple", 3)], &[7; 64]);
        check_eq(format!("{inv:?}"), r#"Inventory { items: {"apple": 3, "pear": 1}, .. }"#)?;
        check_eq(
            format!("{inv:#?}"),
            concat!(
                "Inventory {\n",
                "    items: {\n",
                "        \"apple\": 3,\n",
                "        \"pear\": 1,\n",
                "    },\n",
                "    ..\n",
                "}",
            ),
        )
    });
    case("example-3", || {
        let red = color(255, 0, 16, 255);
        let glass = color(255, 0, 16, 128);
        check_eq(format!("{red:?}"), "Rgb(255, 0, 16)")?;
        check_eq(format!("{red:x?}"), "Rgb(ff, 0, 10)")?;
        check_eq(format!("{glass:?}"), "Rgba(255, 0, 16, 128)")?;
        check_eq(format!("{red:#X?}"), "Rgb(\n    0xFF,\n    0x0,\n    0x10,\n)")
    });
    case("case-1", || {
        // An empty password is redacted too; no scopes; the user name is escaped.
        let c = creds("\"root\"", "", &[]);
        check_eq(
            format!("{c:#?}"),
            concat!(
                "Credentials {\n",
                "    user: \"\\\"root\\\"\",\n",
                "    password: <redacted>,\n",
                "    scopes: [],\n",
                "}",
            ),
        )
    });
    case("case-2", || {
        // No items and an empty cache: the `..` is still there.
        let inv = inventory(&[], &[]);
        check_eq(format!("{inv:#?}"), "Inventory {\n    items: {},\n    ..\n}")
    });
    case("case-3", || {
        // The counts are integers, so they follow the hex flags.
        let inv = inventory(&[("nut", 16), ("bolt", 255)], &[1, 2, 3]);
        check_eq(format!("{inv:x?}"), r#"Inventory { items: {"bolt": ff, "nut": 10}, .. }"#)?;
        check_eq(
            format!("{inv:#X?}"),
            concat!(
                "Inventory {\n",
                "    items: {\n",
                "        \"bolt\": 0xFF,\n",
                "        \"nut\": 0x10,\n",
                "    },\n",
                "    ..\n",
                "}",
            ),
        )
    });
    case("case-4", || {
        // Any alpha other than 255 gives `Rgba`, including 254 and 0.
        check_eq(format!("{:?}", color(1, 2, 3, 254)), "Rgba(1, 2, 3, 254)")?;
        check_eq(format!("{:X?}", color(171, 205, 239, 0)), "Rgba(AB, CD, EF, 0)")?;
        check_eq(format!("{:X?}", color(171, 205, 239, 255)), "Rgb(AB, CD, EF)")
    });
    case("case-5", || {
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
