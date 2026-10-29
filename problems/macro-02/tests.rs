use crate::judge::{case, check_eq};

// Neither `json` nor `Json` is imported here: the macro must work when called by path alone.
type J = crate::user::Json;

fn check_json(got: J, expected: J) -> Result<(), String> {
    check_eq(got, expected)
}

pub fn run() {
    case("example-1", || {
        check_json(
            crate::user::json!({ "a": 1, "b": [true, null, "s"], "c": {} }),
            J::Obj(vec![
                ("a".into(), J::Num(1.0)),
                ("b".into(), J::Arr(vec![J::Bool(true), J::Null, J::Str("s".into())])),
                ("c".into(), J::Obj(vec![])),
            ]),
        )
    });
    case("example-2", || {
        let x: i32 = 4;
        check_json(
            crate::user::json!([(x - 7), "x"]),
            J::Arr(vec![J::Num(-3.0), J::Str("x".into())]),
        )?;
        check_json(crate::user::json!(null), J::Null)
    });
    case("case-1", || {
        // Literals at the top level. The string "null" is not `null`.
        check_json(crate::user::json!(true), J::Bool(true))?;
        check_json(crate::user::json!(false), J::Bool(false))?;
        check_json(crate::user::json!(3), J::Num(3.0))?;
        check_json(crate::user::json!(0), J::Num(0.0))?;
        check_json(crate::user::json!(2.5), J::Num(2.5))?;
        check_json(crate::user::json!(""), J::Str(String::new()))?;
        check_json(crate::user::json!("null"), J::Str("null".into()))?;
        check_json(crate::user::json!("tab\t日本"), J::Str("tab\t日本".into()))
    });
    case("case-2", || {
        // Empty arrays and objects, and trailing commas.
        check_json(crate::user::json!([]), J::Arr(vec![]))?;
        check_json(crate::user::json!({}), J::Obj(vec![]))?;
        check_json(crate::user::json!([1, 2,]), J::Arr(vec![J::Num(1.0), J::Num(2.0)]))?;
        check_json(crate::user::json!({ "k": null, }), J::Obj(vec![("k".into(), J::Null)]))?;
        check_json(
            crate::user::json!([[], {}, [{},], { "e": [], },]),
            J::Arr(vec![
                J::Arr(vec![]),
                J::Obj(vec![]),
                J::Arr(vec![J::Obj(vec![])]),
                J::Obj(vec![("e".into(), J::Arr(vec![]))]),
            ]),
        )
    });
    case("case-3", || {
        // Deep nesting, and arrays and objects with a single element.
        check_json(
            crate::user::json!([[[[[null]]]]]),
            J::Arr(vec![J::Arr(vec![J::Arr(vec![J::Arr(vec![J::Arr(vec![J::Null])])])])]),
        )?;
        check_json(
            crate::user::json!({ "a": { "b": [[false, { "c": [0.5] }]] } }),
            J::Obj(vec![(
                "a".into(),
                J::Obj(vec![(
                    "b".into(),
                    J::Arr(vec![J::Arr(vec![
                        J::Bool(false),
                        J::Obj(vec![("c".into(), J::Arr(vec![J::Num(0.5)]))]),
                    ])]),
                )]),
            )]),
        )
    });
    case("case-4", || {
        // Expressions in parentheses, of every supported type.
        let x: i32 = 4;
        let f = 1.25;
        let name = "Ann";
        let owned = String::from("Bob");
        let inner = crate::user::json!([1, "two"]);
        let expected_inner = J::Arr(vec![J::Num(1.0), J::Str("two".into())]);
        check_json(
            crate::user::json!([
                (-1),
                (x * x),
                (f * 2.0),
                (x > 3),
                (name),
                (owned.clone()),
                (inner.clone()),
                (i32::pow(2, 10)),
            ]),
            J::Arr(vec![
                J::Num(-1.0),
                J::Num(16.0),
                J::Num(2.5),
                J::Bool(true),
                J::Str("Ann".into()),
                J::Str("Bob".into()),
                expected_inner.clone(),
                J::Num(1024.0),
            ]),
        )?;
        check_json(
            crate::user::json!({
                "owned": (owned),
                "inner": (inner),
                "made": (crate::user::json!([(x - 7)])),
            }),
            J::Obj(vec![
                ("owned".into(), J::Str("Bob".into())),
                ("inner".into(), expected_inner),
                ("made".into(), J::Arr(vec![J::Num(-3.0)])),
            ]),
        )
    });
    case("case-5", || {
        // Keys keep their written order, and repeated keys are all kept.
        check_json(
            crate::user::json!({ "z": 1, "a": 2, "m n": 3, "z": [4], "": "" }),
            J::Obj(vec![
                ("z".into(), J::Num(1.0)),
                ("a".into(), J::Num(2.0)),
                ("m n".into(), J::Num(3.0)),
                ("z".into(), J::Arr(vec![J::Num(4.0)])),
                ("".into(), J::Str(String::new())),
            ]),
        )
    });
    case("case-6", || {
        // Every expression is evaluated exactly once, in the order it is written.
        let mut log = Vec::new();
        let mut next = |n: i32| {
            log.push(n);
            n
        };
        let got = crate::user::json!([(next(1)), { "a": (next(2)), "b": [(next(3)), null] }, (next(4))]);
        check_json(
            got,
            J::Arr(vec![
                J::Num(1.0),
                J::Obj(vec![
                    ("a".into(), J::Num(2.0)),
                    ("b".into(), J::Arr(vec![J::Num(3.0), J::Null])),
                ]),
                J::Num(4.0),
            ]),
        )?;
        check_eq(log, vec![1, 2, 3, 4])
    });
}
