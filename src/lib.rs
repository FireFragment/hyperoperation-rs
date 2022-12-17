#![doc = include_str!("../README.md")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

/// Number that can be used either as the first or second number in hyperoperation.
/// Automatically implemented for all fitting numbers.
///
/// # Notable implementors
///
///  - All unsigned primitive numeric types
///  - [BigUint](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html), which is useful to handle big results without overflowing
pub trait NumForKnuth :
    core::ops::Mul<Self, Output = Self> +
    for<'a> core::ops::Mul<&'a Self, Output = Self> +
    core::ops::Sub<Self, Output = Self> +
    std::ops::Add<Output = Self> +
    std::cmp::PartialOrd + Clone +
    num_traits::sign::Unsigned +
    num_traits::One +
    num_traits::ToPrimitive {}


impl<Num> NumForKnuth for Num
where
    Num:
        core::ops::Mul<Num, Output = Num> +
        for<'a> core::ops::Mul<&'a Self, Output = Self> +
        core::ops::Sub<Num, Output = Num> +
        std::ops::Add<Output = Num> +
        std::cmp::PartialOrd + Clone +
        num_traits::sign::Unsigned +
        num_traits::One +
        num_traits::ToPrimitive,

    for<'a> &'a Num: core::ops::Mul<&'a Num, Output = Num> {}

/// Calculate result of hyperoperation
///
/// First argument is the first number, second argument is the second number. Third argument is number of arrows in Knuth's up-arrow notation. \
/// **Example:** `hyperoperation(4, 3, 2)` corresponds to 4 ↑↑ 3
///
/// This function is equivalent to `KnuthNotation::new(num_a, num_b, arrows).evaluate()`. [More about Hyperoperation struct...](Hyperoperation)
///
/// ```
/// # use hyperoperation::hyperoperation;
/// assert_eq!(
///     hyperoperation::<u64>(&3, 3, 2), // 3 ↑↑ 3
///     7625597484987
/// );
/// ```
pub fn hyperoperation<Num: NumForKnuth>(num_a: &Num, num_b: Num, arrows: u8) -> Num {
    // TODO: Use power
    if arrows == 0 {
        num_b * num_a
    } else {
        let mut res = num_a.clone();
        let max = num_b - Num::one();
        for _ in num_iter::range_inclusive(Num::one(), max) {
            res = hyperoperation(num_a, res, arrows - 1);
        }
        res
    }
}

/// Representation of Hyperoperation
///
/// # Features
///
///  - Evaluate the operation with [evaluate](Self::evaluate)
///  - Format it with the Knuth's up-arrow notation
///
/// # Example
///
/// Evaluating hyperoperation and formatting it with [Knuth's up-arrow notation](https://en.wikipedia.org/wiki/Knuth%27s_up-arrow_notation):
/// ```
/// # use hyperoperation::Hyperoperation;
/// let expr = Hyperoperation::<u64>::new(3, 3, 2); // Represents 3 ↑↑ 3
/// let result = expr.clone().evaluate(); // Calculate the value of 3 ↑↑ 3
///
/// println!("{expr} = {result}");
/// assert_eq!(result, 7625597484987);
/// assert_eq!(format!("{expr}"), "3 ↑↑ 3");
/// ```
#[derive(Clone)]
pub struct Hyperoperation<Num: NumForKnuth>
{
    /// The first number, _before_ the arrows in Knuth's up-arrow notation
    pub num_a: Num,

    /// The second numer, _after_ the arrows in Knuth's up-arrow notation
    pub num_b: Num,

    /// Number of arrows in Knuth's up-arrow notation
    pub arrows: u8
}

impl<Num: NumForKnuth> Hyperoperation<Num> {
    /// Calculates the value of the operation.
    ///
    /// Please keep in mind, that for some expressions (like 3 ↑↑↑ 3), this could take a lot of time and/or overflow the value. \
    /// To correctly handle large results, it's recommended to use [BigUint](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html) as `Num`.
    ///
    /// # Panics
    ///
    /// In debug mode, the result might overflow `Num`'s capacity. In release mode, **it might silently overflow**!
    ///
    /// # Example
    ///
    /// ```
    /// # use hyperoperation::Hyperoperation;
    /// let expr = Hyperoperation::<u64>::new(3, 3, 2); // Represents 3 ↑↑ 3
    /// assert_eq!(expr.evaluate(), 7625597484987);
    /// ```
    pub fn evaluate(self) -> Num {
        hyperoperation(&self.num_a, self.num_b, self.arrows)
    }

    /// Shorthand for initializing the struct
    pub fn new(num_a: Num, num_b: Num, arrows: u8) -> Self {
        Self {
            num_a, num_b, arrows
        }
    }
}

/*
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
///     knuth!(u8, 3 ^2^ 2).evaluate(),
///     27
/// );
/// ```
#[macro_export]
macro_rules! knuth {
    ($type:ty, $a:literal ^$arrs:literal^ $b:literal) => {
    {
        $crate::KnuthNotation {
            num_a: <$type>::from($a as u128),
            num_b: <$type>::from($b as u128),
            arrows: $arrs
        }
    }};
}*/

impl<Num: core::fmt::Display + NumForKnuth> std::fmt::Display for Hyperoperation<Num> {
    /// Format the expression as Knuth's notation
    ///
    /// # Example
    ///
    /// ```
    /// # use hyperoperation::Hyperoperation;
    /// assert_eq!(format!("{}", Hyperoperation::<u32> {num_a: 3, num_b: 4, arrows: 2} ), String::from("3 ↑↑ 4"))
    /// ```
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
    use num_bigint::BigUint;
    use super::*;

    type KN = Hyperoperation<BigUint>;
    #[test]
    fn small() {
        let exprs: Vec<(KN, BigUint)> = vec![
            (KN::new(4u8.into(), 7u8.into(), 0), 28u32.into()),
            (KN::new(3u8.into(), 2u8.into(), 2), 27u8.into()),
            (KN::new(2u8.into(), 4u8.into(), 2), 65536u32.into()),
            (KN::new(2u8.into(), 3u8.into(), 3), 65536u32.into()),
            (KN::new(3u8.into(), 3u8.into(), 2), 7625597484987u64.into()),
        ];

        for (ex, res) in exprs {
            println!("Evaluating {ex}");
            assert_eq!(ex.evaluate(), res);
        }
    }

    #[test]
    fn biguint() {
        let result = KN::new(5u8.into(), 3u8.into(), 2).evaluate();
        println!("Result:\n{result}");
        assert_eq!(
            result % BigUint::from(100_000_000u32),
            8203125u32.into()
        )
    }
}
