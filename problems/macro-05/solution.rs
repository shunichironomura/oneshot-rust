// max_of!(3, 9, 4) is 9.
macro_rules! max_of {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        let max = $first;
        $(
            let next = $rest;
            let max = if next > max { next } else { max };
        )*
        max
    }};
}
pub(crate) use max_of;
