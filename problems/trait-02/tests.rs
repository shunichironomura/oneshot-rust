use crate::judge::{case, check_eq};
use crate::user::Pairs;

pub fn run() {
    case("sample-1", || {
        let got: Vec<(i32, i32)> = vec![1, 2, 3, 4].into_iter().pairs().collect();
        check_eq(got, vec![(1, 2), (2, 3), (3, 4)])
    });
    case("sample-2", || {
        let got: Vec<(char, char)> = "abc".chars().pairs().collect();
        check_eq(got, vec![('a', 'b'), ('b', 'c')])
    });
    case("hidden-1", || {
        let got: Vec<(u8, u8)> = Vec::<u8>::new().into_iter().pairs().collect();
        check_eq(got, vec![])
    });
    case("hidden-2", || check_eq(std::iter::once(7).pairs().next(), None));
    case("hidden-3", || {
        let words = vec![String::from("x"), String::from("y"), String::from("z")];
        let got: Vec<(String, String)> = words.into_iter().pairs().collect();
        let expected = vec![
            (String::from("x"), String::from("y")),
            (String::from("y"), String::from("z")),
        ];
        check_eq(got, expected)
    });
    case("hidden-4", || {
        let got: Vec<(u32, u32)> = (0..).pairs().take(3).collect();
        check_eq(got, vec![(0, 1), (1, 2), (2, 3)])
    });
    case("hidden-5", || {
        let values = [5, 1, 4];
        let got: Vec<(&i32, &i32)> = values.iter().pairs().collect();
        check_eq(got, vec![(&5, &1), (&1, &4)])
    });
}
