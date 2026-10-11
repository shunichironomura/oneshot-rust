use crate::judge::{case, check_eq};
use crate::user::{Kind, Rng, Token, classify, score};

fn draws(rng: &mut Rng, n: usize) -> Vec<u32> {
    (0..n).map(|_| rng.r#gen()).collect()
}

fn token(kind: Kind, text: &str) -> Token {
    Token { r#type: kind, text: text.to_string() }
}

pub fn run() {
    case("sample-1", || {
        check_eq(draws(&mut Rng::new(0), 3), vec![167951807, 218396424, 1299921937])
    });
    case("sample-2", || {
        check_eq(classify("gen"), token(Kind::Keyword, "gen"))?;
        check_eq(classify("gene").r#type, Kind::Ident)
    });
    case("sample-3", || check_eq(score(vec![3, 4].into_boxed_slice()), 9));
    case("hidden-1", || {
        check_eq(draws(&mut Rng::new(1), 3), vec![908834774, 1093944153, 1392341196])?;
        check_eq(draws(&mut Rng::new(u64::MAX), 3), vec![1574552488, 1490332343, 1207502677])
    });
    case("hidden-2", || {
        // gen_below draws one number with gen and reduces it modulo bound.
        let mut a = Rng::new(2024);
        let mut b = Rng::new(2024);
        for bound in [10, 7, u32::MAX, 2, 1000, 1] {
            check_eq(a.gen_below(bound), b.r#gen() % bound)?;
        }
        check_eq(a.r#gen(), b.r#gen())?;
        let mut c = Rng::new(5);
        for _ in 0..10 {
            check_eq(c.gen_below(1), 0)?;
        }
        Ok(())
    });
    case("hidden-3", || {
        for word in ["fn", "let", "match", "async", "try"] {
            check_eq(classify(word), token(Kind::Keyword, word))?;
        }
        Ok(())
    });
    case("hidden-4", || {
        // Only exact, case-sensitive matches are keywords.
        for word in ["Fn", "GEN", "", "fn ", "r#gen", "lets", "mat"] {
            check_eq(classify(word), token(Kind::Ident, word))?;
        }
        Ok(())
    });
    case("hidden-5", || {
        check_eq(score(Vec::new().into_boxed_slice()), 0)?;
        check_eq(score(vec![0].into_boxed_slice()), 1)?;
        check_eq(score(vec![10, 0, 25, 4, 1].into_boxed_slice()), 45)
    });
}
