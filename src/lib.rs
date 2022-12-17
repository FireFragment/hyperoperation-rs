use num_bigint::BigUint;
pub use num_bigint;

// TODO: Make it generic

/// Evaluate number expressed with knuth up-arrow notation. \
/// Please prefer using [knuth!] macro when dealing with literals.
pub fn knuth(num_a: &BigUint, num_b: &BigUint, arrows: u8) -> BigUint {
    if arrows == 0 {
        num_a * num_b
    } else {
        let mut res = num_a.clone();
        let max = num_b.clone() - 1u32;
        for _ in num_iter::range_inclusive(BigUint::from(1u32), max.clone()) {
            res = knuth(num_a, &res, arrows - 1);
        }
        res
    }
}

pub struct KnuthNotation {
    pub num_a: BigUint,
    pub num_b: BigUint,

    /// Number of arrows in the knuth's notation.
    pub arrows: u8
}

impl KnuthNotation {
    /// Calculate the value of the expression. \
    /// Please keep in mind, that for some expressions (like 3 ↑↑↑ 3), this could take a lot of time.
    pub fn evaluate(&self) -> BigUint {
        knuth(&self.num_a, &self.num_b, self.arrows)
    }
}

/// A more readable way of initializing the [KnuthNotation] struct. \
/// It should be the preferred way to initialize it. Evaluate it with the [`evaluate`](KnuthNotation::evaluate) function
///
/// It's not possible to use expressions in this macro. If you need to, initialize [KnuthNotation] directly
/// # Examples
///
/// | Syntax | Knuth's up-arrow notation |
/// | ------ | ------- |
/// | `knuth!(3 ^2^ 2)` | 3 ↑↑ 2 |
/// | `knuth!(3 ^2^ 4)` | 3 ↑↑ 4 |
/// | `knuth!(3 ^3^ 2)` | 3 ↑↑↑ 2 |
///
/// Example of usage:
/// ```
/// use knuth::knuth;
/// // Evaluating the notation
/// assert_eq!(
///     knuth!(3 ^2^ 2).evaluate(),
///     27u8.into()
/// );
/// ```
#[macro_export]
macro_rules! knuth {
    ($a:literal ^$arrs:literal^ $b:literal) => {
    {
        $crate::KnuthNotation {
            num_a: $crate::num_bigint::BigUint::new(vec![$a]),
            num_b: $crate::num_bigint::BigUint::new(vec![$b]),
            arrows: $arrs
        }
    }};
}

impl std::fmt::Display for KnuthNotation {
    /// Format the expression as Knuth's notation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrs = match self.arrows {
            0 => String::from("×"),
            _ => "↑".repeat((self.arrows)as usize)
        };
        write!(f, "{} {arrs} {}", self.num_a, self.num_b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let exprs: Vec<(_, BigUint)> = vec![
            (knuth!(4 ^0^ 7), 28u32.into()),
            (knuth!(3 ^2^ 2), 27u8.into()),
            (knuth!(2 ^2^ 4), 65536u32.into()),
            (knuth!(2 ^3^ 3), 65536u32.into()),
            (knuth!(3 ^3^ 2), 7625597484987u64.into()),
        ];

        for (ex, res) in exprs {
            println!("Evaluating {ex}");
            assert_eq!(ex.evaluate(), res);
        }
    }
}
