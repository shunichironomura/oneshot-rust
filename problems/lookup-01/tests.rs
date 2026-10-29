use crate::judge::{case, check, check_eq};
use crate::user::Registry;

fn fruits() -> Registry<String, u32> {
    let mut r = Registry::new();
    r.insert(String::from("apple"), 1);
    r.insert(String::from("pear"), 2);
    r
}

pub fn run() {
    case("example-1", || {
        // A String key can be looked up with a &str or a &String.
        let mut r = fruits();
        check_eq(r.get("apple"), Some(&1))?;
        check_eq(r.get(&String::from("pear")), Some(&2))?;
        check(r.contains("pear"), "contains(\"pear\") should be true")?;
        check_eq(r.remove("apple"), Some(1))?;
        check_eq(r.get("apple"), None)?;
        let zero = 0;
        check_eq(*r.get_or("zzz", &zero), 0)?;
        check_eq(*r.get_or("pear", &zero), 2)
    });
    case("example-2", || {
        // A Vec<u8> key is looked up with a &[u8], a u32 key with a &u32.
        let mut bytes: Registry<Vec<u8>, &str> = Registry::new();
        bytes.insert(vec![1, 2], "one-two");
        check_eq(bytes.get(&[1u8, 2][..]), Some(&"one-two"))?;
        let mut chars: Registry<u32, char> = Registry::new();
        chars.insert(7, 'x');
        check_eq(chars.get(&7), Some(&'x'))
    });
    case("case-1", || {
        // Missing keys, then replacing and removing a value.
        let mut r = fruits();
        check_eq(r.get("kiwi"), None)?;
        check(!r.contains(""), "contains(\"\") should be false")?;
        check_eq(r.remove(&String::from("pea")), None)?;
        r.insert(String::from("apple"), 10);
        check_eq(r.get("apple"), Some(&10))?;
        check_eq(r.remove(&String::from("apple")), Some(10))?;
        check(!r.contains("apple"), "\"apple\" should be gone")
    });
    case("case-2", || {
        // The key may be dropped before the result is used,
        // and the default may live shorter than the registry.
        let r = fruits();
        let found = {
            let key = String::from("pe") + "ar";
            r.get(key.as_str())
        };
        check_eq(found, Some(&2))?;
        let zero = 0;
        let missing = r.get_or(&String::from("plum"), &zero);
        let present = r.get_or(&String::from("apple"), &zero);
        check_eq((*missing, *present), (0, 1))?;
        let n = {
            let fallback = 99;
            let got = r.get_or("kiwi", &fallback);
            *got
        };
        check_eq(n, 99)
    });
    case("case-3", || {
        // Byte-string keys (including the empty one) and u32 keys.
        let mut bytes: Registry<Vec<u8>, &str> = Registry::new();
        bytes.insert(Vec::new(), "empty");
        bytes.insert(vec![1, 2, 3], "three");
        let empty: &[u8] = &[];
        check_eq(bytes.get(empty), Some(&"empty"))?;
        check_eq(bytes.get(&[1u8, 2][..]), None)?;
        check(bytes.contains(&vec![1u8, 2, 3]), "contains(&vec![1, 2, 3]) should be true")?;
        check_eq(bytes.remove(&[1u8, 2, 3][..]), Some("three"))?;
        let unknown = String::from("unknown");
        let fallback = unknown.as_str();
        check_eq(*bytes.get_or(&[1u8, 2, 3][..], &fallback), "unknown")?;

        let mut chars: Registry<u32, char> = Registry::new();
        chars.insert(7, 'x');
        check(!chars.contains(&8), "contains(&8) should be false")?;
        check_eq(chars.remove(&7), Some('x'))?;
        let dash = '-';
        check_eq(*chars.get_or(&7, &dash), '-')
    });
}
