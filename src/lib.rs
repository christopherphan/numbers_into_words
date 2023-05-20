/* src/lib.rs
 *
 * This file is part of numbers_into_words
 *
 * Copyright (C) 2023 Christopher Phan
 * https://chrisphan.com/
 *
 * Licensed under MIT or APACHE 2.0
 *
 * See LICENSE-MIT.txt and LICENSE-APACHE-2.0.txt
 * in repository root directory.
 * */

/// Copyright info for this crate
pub const COPYRIGHT_INFO: &str = "\
         Copyright \u{00a9} 2023 Christopher Phan\n\
         https://chrisphan.com/\n\
         Licensed under MIT or APACHE 2.0";

pub use conversion_to_words::to_word;
pub use conversion_to_words::AndBehavior;
pub use process_input::Config;

pub mod conversion_to_words {
    const AND_STR: &str = " and ";

    /// Signals when the word "and" should be used in an output
    #[derive(Copy, Clone)]
    pub enum AndBehavior {
        /// Indicates that the word "and" is not to be used.
        ///
        /// # Examples
        ///
        /// ```
        /// use numbers_into_words::{to_word, AndBehavior};
        /// assert_eq!(
        ///     to_word(350_000_000, AndBehavior::None),
        ///     "three-hundred fifty million".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(350_000_430, AndBehavior::None),
        ///     "three-hundred fifty million, four-hundred thirty".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(2_859, AndBehavior::None),
        ///     "two thousand, eight-hundred fifty-nine".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(731, AndBehavior::None),
        ///     "seven-hundred thirty-one".to_string()
        /// );
        /// ```
        None,

        /// Indicates that the word "and" is to be used only in last group (hundreds, tens, and one
        /// digit).
        ///
        /// # Examples
        ///
        /// ```
        /// use numbers_into_words::{to_word, AndBehavior};
        /// assert_eq!(
        ///     to_word(350_000_000, AndBehavior::LastGroup),
        ///     "three-hundred fifty million".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(350_000_430, AndBehavior::LastGroup),
        ///     "three-hundred fifty million, four-hundred and thirty".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(2_859, AndBehavior::LastGroup),
        ///     "two thousand, eight-hundred and fifty-nine".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(731, AndBehavior::LastGroup),
        ///     "seven-hundred and thirty-one".to_string()
        /// );
        /// ```
        LastGroup,

        /// Indicates that the word "and" is to be used only
        /// if the number is less than 1000.
        ///
        /// # Examples
        ///
        /// ```
        /// use numbers_into_words::{to_word, AndBehavior};
        /// assert_eq!(
        ///     to_word(350_000_000, AndBehavior::OnlyUnderThousand),
        ///     "three-hundred fifty million".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(350_000_430, AndBehavior::OnlyUnderThousand),
        ///     "three-hundred fifty million, four-hundred thirty".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(2_859, AndBehavior::OnlyUnderThousand),
        ///     "two thousand, eight-hundred fifty-nine".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(731, AndBehavior::OnlyUnderThousand),
        ///     "seven-hundred and thirty-one".to_string()
        /// );
        /// ```
        OnlyUnderThousand,

        /// Indicates that the word "and" is to be used in every group.
        ///
        /// # Examples
        ///
        /// ```
        /// use numbers_into_words::{to_word, AndBehavior};
        /// assert_eq!(
        ///     to_word(350_000_000, AndBehavior::All),
        ///     "three-hundred and fifty million".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(350_000_430, AndBehavior::All),
        ///     "three-hundred and fifty million, four-hundred and thirty".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(2_859, AndBehavior::All),
        ///     "two thousand, eight-hundred and fifty-nine".to_string()
        /// );
        /// assert_eq!(
        ///     to_word(731, AndBehavior::All),
        ///     "seven-hundred and thirty-one".to_string()
        /// );
        /// ```
        All,
    }

    impl AndBehavior {
        fn insert_and(&self, group: usize, value: u64) -> &'static str {
            match (self, group, value) {
                (Self::None, _, _) => " ",
                (Self::LastGroup, 0, _) => AND_STR,
                (Self::LastGroup, _, _) => " ",
                (Self::OnlyUnderThousand, _, 0..=999) => AND_STR,
                (Self::OnlyUnderThousand, _, _) => " ",
                (Self::All, _, _) => AND_STR,
            }
        }
    }

    fn single_digit(x: u64) -> Result<String, &'static str> {
        match x {
            0 => Ok("zero".to_string()),
            1 => Ok("one".to_string()),
            2 => Ok("two".to_string()),
            3 => Ok("three".to_string()),
            4 => Ok("four".to_string()),
            5 => Ok("five".to_string()),
            6 => Ok("six".to_string()),
            7 => Ok("seven".to_string()),
            8 => Ok("eight".to_string()),
            9 => Ok("nine".to_string()),
            _ => Err("Value over 9."),
        }
    }

    fn under_100(x: u64) -> Result<String, &'static str> {
        match x {
            0..=9 => single_digit(x),
            10 => Ok("ten".to_string()),
            11 => Ok("eleven".to_string()),
            12 => Ok("twelve".to_string()),
            13 => Ok("thirteen".to_string()),
            15 => Ok("fifteen".to_string()),
            18 => Ok("eighteen".to_string()),
            14 | 16 | 17 | 19 => Ok(format!("{}teen", single_digit(x % 10).expect("under 10"))),
            20 => Ok("twenty".to_string()),
            30 => Ok("thirty".to_string()),
            40 => Ok("forty".to_string()),
            50 => Ok("fifty".to_string()),
            80 => Ok("eighty".to_string()),
            x if x % 10 == 0 => Ok(format!("{}ty", single_digit(x / 10).expect("under 10"))),
            21..=99 => Ok(format!(
                "{}-{}",
                under_100(x - (x % 10)).expect("under 100"),
                single_digit(x % 10).expect("under 10")
            )),
            _ => Err("Value over 99"),
        }
    }

    fn under_1000(
        x: u64,
        group: usize,
        and_behavior: AndBehavior,
        full_value: u64,
    ) -> Result<String, &'static str> {
        match x {
            0..=99 => under_100(x),
            100..=900 if x % 100 == 0 => Ok(format!(
                "{}-hundred",
                single_digit(x / 100).expect("under 10")
            )),
            x if x < 1000 => Ok(format!(
                "{}{}{}",
                under_1000(x - (x % 100), group, and_behavior, full_value).expect("under 1000"),
                and_behavior.insert_and(group, full_value),
                under_100(x % 100).expect("under 100")
            )),
            _ => Err("Value over 999."),
        }
    }

    const POWERS_THOUSAND: [&str; 7] = [
        "",
        " thousand",
        " million",
        " billion",
        " trillion",
        " quadrillion",
        " quintillion",
    ];

    /// Convert a 64-bit unsigned integer to words
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_into_words::{to_word, AndBehavior};
    ///
    /// // US population according to 2020 census
    /// // https://www2.census.gov/library/publications/decennial/2020/census-briefs/c2020br-01.pdf
    ///
    /// assert_eq!(
    ///     to_word(330_759_736, AndBehavior::All),
    ///     String::from(
    ///         "three-hundred and thirty million, \
    ///         seven-hundred and fifty-nine thousand, \
    ///         seven-hundred and thirty-six"
    ///     )
    /// );
    /// assert_eq!(
    ///     to_word(330_759_736, AndBehavior::LastGroup),
    ///     String::from(
    ///         "three-hundred thirty million, \
    ///         seven-hundred fifty-nine thousand, \
    ///         seven-hundred and thirty-six"
    ///     )
    /// );
    ///
    /// assert_eq!(
    ///     to_word(330_759_736, AndBehavior::OnlyUnderThousand),
    ///     String::from(
    ///         "three-hundred thirty million, \
    ///         seven-hundred fifty-nine thousand, \
    ///         seven-hundred thirty-six"
    ///     )
    /// );
    ///
    /// assert_eq!(
    ///     to_word(123, AndBehavior::OnlyUnderThousand),
    ///     String::from("one-hundred and twenty-three")
    /// );
    ///
    /// assert_eq!(
    ///     to_word(123, AndBehavior::None),
    ///     String::from("one-hundred twenty-three")
    /// );
    ///
    /// assert_eq!(to_word(0, AndBehavior::None), "zero".to_string());
    /// ```
    pub fn to_word(x: u64, and_behavior: AndBehavior) -> String {
        if x == 0 {
            single_digit(0).expect("under 10")
        } else {
            (0..7)
                .map(|y| ((x / (10_u64).pow(3 * (6 - y as u32))) % 1000, 6 - y, x))
                .filter(|(a, _, _)| *a != 0_u64)
                .map(|(a, b, x)| {
                    format!(
                        "{}{}",
                        under_1000(a, b, and_behavior, x).expect("under 1000"),
                        POWERS_THOUSAND[b]
                    )
                })
                .collect::<Vec<String>>()
                .join(", ")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_single_digit() {
            assert_eq!(single_digit(0).unwrap(), String::from("zero"));
            assert_eq!(single_digit(5).unwrap(), String::from("five"));
            assert!(single_digit(14).is_err());
        }

        #[test]
        fn test_under_100() {
            assert_eq!(under_100(3).unwrap(), String::from("three"));
            assert_eq!(under_100(12).unwrap(), String::from("twelve"));
            assert_eq!(under_100(19).unwrap(), String::from("nineteen"));
            assert_eq!(under_100(20).unwrap(), String::from("twenty"));
            assert_eq!(under_100(47).unwrap(), String::from("forty-seven"));
            assert!(under_100(105).is_err());
        }

        #[test]
        fn test_under_1000() {
            assert_eq!(
                under_1000(3, 0, AndBehavior::All, 3).unwrap(),
                String::from("three")
            );
            assert_eq!(
                under_1000(12, 0, AndBehavior::All, 12).unwrap(),
                String::from("twelve")
            );
            assert_eq!(
                under_1000(19, 0, AndBehavior::All, 19).unwrap(),
                String::from("nineteen")
            );
            assert_eq!(
                under_1000(20, 0, AndBehavior::All, 20).unwrap(),
                String::from("twenty")
            );
            assert_eq!(
                under_1000(47, 0, AndBehavior::All, 47).unwrap(),
                String::from("forty-seven")
            );
            assert_eq!(
                under_1000(120, 0, AndBehavior::All, 120).unwrap(),
                String::from("one-hundred and twenty")
            );
            assert_eq!(
                under_1000(247, 0, AndBehavior::All, 247).unwrap(),
                String::from("two-hundred and forty-seven")
            );
            assert_eq!(
                under_1000(403, 0, AndBehavior::All, 403).unwrap(),
                String::from("four-hundred and three")
            );
            assert_eq!(
                under_1000(612, 0, AndBehavior::All, 612).unwrap(),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                under_1000(919, 0, AndBehavior::All, 919).unwrap(),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                under_1000(612, 0, AndBehavior::LastGroup, 234612).unwrap(),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                under_1000(919, 0, AndBehavior::LastGroup, 239919).unwrap(),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                under_1000(120, 1, AndBehavior::LastGroup, 120330).unwrap(),
                String::from("one-hundred twenty")
            );
            assert_eq!(
                under_1000(247, 1, AndBehavior::LastGroup, 247123).unwrap(),
                String::from("two-hundred forty-seven")
            );
            assert_eq!(
                under_1000(612, 0, AndBehavior::OnlyUnderThousand, 612).unwrap(),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                under_1000(919, 0, AndBehavior::OnlyUnderThousand, 919).unwrap(),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                under_1000(120, 1, AndBehavior::OnlyUnderThousand, 120330).unwrap(),
                String::from("one-hundred twenty")
            );
            assert_eq!(
                under_1000(3, 0, AndBehavior::None, 3).unwrap(),
                String::from("three")
            );
            assert_eq!(
                under_1000(12, 0, AndBehavior::None, 12).unwrap(),
                String::from("twelve")
            );
            assert_eq!(
                under_1000(19, 0, AndBehavior::None, 19).unwrap(),
                String::from("nineteen")
            );
            assert_eq!(
                under_1000(20, 0, AndBehavior::None, 20).unwrap(),
                String::from("twenty")
            );
            assert_eq!(
                under_1000(47, 0, AndBehavior::None, 47).unwrap(),
                String::from("forty-seven")
            );
            assert_eq!(
                under_1000(120, 0, AndBehavior::None, 120).unwrap(),
                String::from("one-hundred twenty")
            );
            assert_eq!(
                under_1000(247, 0, AndBehavior::None, 247).unwrap(),
                String::from("two-hundred forty-seven")
            );
            assert_eq!(
                under_1000(403, 0, AndBehavior::None, 403).unwrap(),
                String::from("four-hundred three")
            );
            assert_eq!(
                under_1000(612, 0, AndBehavior::None, 612).unwrap(),
                String::from("six-hundred twelve")
            );
            assert_eq!(
                under_1000(919, 0, AndBehavior::None, 919).unwrap(),
                String::from("nine-hundred nineteen")
            );
            assert!(under_1000(2105, 0, AndBehavior::All, 2105).is_err());
            assert!(under_1000(2105, 0, AndBehavior::None, 2105).is_err());
            assert!(under_1000(2105, 0, AndBehavior::LastGroup, 2105).is_err());
            assert!(under_1000(2105, 0, AndBehavior::OnlyUnderThousand, 2105).is_err());
        }

        #[test]
        fn test_to_word() {
            assert_eq!(to_word(0, AndBehavior::None), String::from("zero"));
            assert_eq!(to_word(3, AndBehavior::None), String::from("three"));
            assert_eq!(to_word(12, AndBehavior::None), String::from("twelve"));
            assert_eq!(to_word(19, AndBehavior::None), String::from("nineteen"));
            assert_eq!(to_word(20, AndBehavior::None), String::from("twenty"));
            assert_eq!(to_word(47, AndBehavior::None), String::from("forty-seven"));
            assert_eq!(
                to_word(120, AndBehavior::None),
                String::from("one-hundred twenty")
            );
            assert_eq!(
                to_word(247, AndBehavior::None),
                String::from("two-hundred forty-seven")
            );
            assert_eq!(
                to_word(403, AndBehavior::None),
                String::from("four-hundred three")
            );
            assert_eq!(
                to_word(612, AndBehavior::None),
                String::from("six-hundred twelve")
            );
            assert_eq!(
                to_word(919, AndBehavior::None),
                String::from("nine-hundred nineteen")
            );
            assert_eq!(
                to_word(2105, AndBehavior::None),
                String::from("two thousand, one-hundred five")
            );
            assert_eq!(
                to_word(200_105, AndBehavior::None),
                String::from("two-hundred thousand, one-hundred five")
            );
            assert_eq!(
                to_word(530_175_000, AndBehavior::None),
                String::from("five-hundred thirty million, one-hundred seventy-five thousand")
            );
            assert_eq!(
                to_word(530_175_999, AndBehavior::None),
                String::from(
                    "five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_530_175_999, AndBehavior::None),
                String::from(
                    "four billion, five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_000_175_999, AndBehavior::None),
                String::from(
                    "four billion, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(14_000_001_019, AndBehavior::None),
                String::from("fourteen billion, one thousand, nineteen")
            );
            assert_eq!(
                to_word(123_456_789_012_345, AndBehavior::None),
                String::from(
                    "one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(17_654_123_456_789_012_345, AndBehavior::None),
                String::from(
                    "seventeen quintillion, six-hundred fifty-four quadrillion, \
                    one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(u64::MAX, AndBehavior::None),
                String::from(
                    "eighteen quintillion, four-hundred forty-six quadrillion, \
                 seven-hundred forty-four trillion, seventy-three billion, \
                 seven-hundred nine million, five-hundred fifty-one thousand, \
                 six-hundred fifteen"
                )
            );

            assert_eq!(to_word(0, AndBehavior::All), String::from("zero"));
            assert_eq!(to_word(3, AndBehavior::All), String::from("three"));
            assert_eq!(to_word(12, AndBehavior::All), String::from("twelve"));
            assert_eq!(to_word(19, AndBehavior::All), String::from("nineteen"));
            assert_eq!(to_word(20, AndBehavior::All), String::from("twenty"));
            assert_eq!(to_word(47, AndBehavior::All), String::from("forty-seven"));
            assert_eq!(
                to_word(120, AndBehavior::All),
                String::from("one-hundred and twenty")
            );
            assert_eq!(
                to_word(247, AndBehavior::All),
                String::from("two-hundred and forty-seven")
            );
            assert_eq!(
                to_word(403, AndBehavior::All),
                String::from("four-hundred and three")
            );
            assert_eq!(
                to_word(612, AndBehavior::All),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                to_word(919, AndBehavior::All),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                to_word(2105, AndBehavior::All),
                String::from("two thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(200_105, AndBehavior::All),
                String::from("two-hundred thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(530_175_000, AndBehavior::All),
                String::from(
                    "five-hundred and thirty million, one-hundred and seventy-five thousand"
                )
            );
            assert_eq!(
                to_word(530_175_999, AndBehavior::All),
                String::from(
                    "five-hundred and thirty million, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_530_175_999, AndBehavior::All),
                String::from(
                    "four billion, five-hundred and thirty million, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_000_175_999, AndBehavior::All),
                String::from(
                    "four billion, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(14_000_001_019, AndBehavior::All),
                String::from("fourteen billion, one thousand, nineteen")
            );
            assert_eq!(
                to_word(123_456_789_012_345, AndBehavior::All),
                String::from(
                    "one-hundred and twenty-three trillion, four-hundred and fifty-six billion, \
                      seven-hundred and eighty-nine million, twelve thousand, three-hundred and \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(17_654_123_456_789_012_345, AndBehavior::All),
                String::from(
                    "seventeen quintillion, six-hundred and fifty-four quadrillion, \
                    one-hundred and twenty-three trillion, four-hundred and fifty-six billion, \
                      seven-hundred and eighty-nine million, twelve thousand, three-hundred and \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(u64::MAX, AndBehavior::All),
                String::from(
                    "eighteen quintillion, four-hundred and forty-six quadrillion, \
                 seven-hundred and forty-four trillion, seventy-three billion, \
                 seven-hundred and nine million, five-hundred and fifty-one thousand, \
                 six-hundred and fifteen"
                )
            );

            assert_eq!(
                to_word(0, AndBehavior::OnlyUnderThousand),
                String::from("zero")
            );
            assert_eq!(
                to_word(3, AndBehavior::OnlyUnderThousand),
                String::from("three")
            );
            assert_eq!(
                to_word(12, AndBehavior::OnlyUnderThousand),
                String::from("twelve")
            );
            assert_eq!(
                to_word(19, AndBehavior::OnlyUnderThousand),
                String::from("nineteen")
            );
            assert_eq!(
                to_word(20, AndBehavior::OnlyUnderThousand),
                String::from("twenty")
            );
            assert_eq!(
                to_word(47, AndBehavior::OnlyUnderThousand),
                String::from("forty-seven")
            );
            assert_eq!(
                to_word(120, AndBehavior::OnlyUnderThousand),
                String::from("one-hundred and twenty")
            );
            assert_eq!(
                to_word(247, AndBehavior::OnlyUnderThousand),
                String::from("two-hundred and forty-seven")
            );
            assert_eq!(
                to_word(403, AndBehavior::OnlyUnderThousand),
                String::from("four-hundred and three")
            );
            assert_eq!(
                to_word(612, AndBehavior::OnlyUnderThousand),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                to_word(919, AndBehavior::OnlyUnderThousand),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                to_word(2105, AndBehavior::OnlyUnderThousand),
                String::from("two thousand, one-hundred five")
            );
            assert_eq!(
                to_word(200_105, AndBehavior::OnlyUnderThousand),
                String::from("two-hundred thousand, one-hundred five")
            );
            assert_eq!(
                to_word(530_175_000, AndBehavior::OnlyUnderThousand),
                String::from("five-hundred thirty million, one-hundred seventy-five thousand")
            );
            assert_eq!(
                to_word(530_175_999, AndBehavior::OnlyUnderThousand),
                String::from(
                    "five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_530_175_999, AndBehavior::OnlyUnderThousand),
                String::from(
                    "four billion, five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_000_175_999, AndBehavior::OnlyUnderThousand),
                String::from(
                    "four billion, one-hundred \
                         seventy-five thousand, nine-hundred ninety-nine"
                )
            );
            assert_eq!(
                to_word(14_000_001_019, AndBehavior::OnlyUnderThousand),
                String::from("fourteen billion, one thousand, nineteen")
            );
            assert_eq!(
                to_word(123_456_789_012_345, AndBehavior::OnlyUnderThousand),
                String::from(
                    "one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(17_654_123_456_789_012_345, AndBehavior::OnlyUnderThousand),
                String::from(
                    "seventeen quintillion, six-hundred fifty-four quadrillion, \
                    one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(u64::MAX, AndBehavior::OnlyUnderThousand),
                String::from(
                    "eighteen quintillion, four-hundred forty-six quadrillion, \
                 seven-hundred forty-four trillion, seventy-three billion, \
                 seven-hundred nine million, five-hundred fifty-one thousand, \
                 six-hundred fifteen"
                )
            );
            assert_eq!(to_word(0, AndBehavior::LastGroup), String::from("zero"));
            assert_eq!(to_word(3, AndBehavior::LastGroup), String::from("three"));
            assert_eq!(to_word(12, AndBehavior::LastGroup), String::from("twelve"));
            assert_eq!(
                to_word(19, AndBehavior::LastGroup),
                String::from("nineteen")
            );
            assert_eq!(to_word(20, AndBehavior::LastGroup), String::from("twenty"));
            assert_eq!(
                to_word(47, AndBehavior::LastGroup),
                String::from("forty-seven")
            );
            assert_eq!(
                to_word(120, AndBehavior::LastGroup),
                String::from("one-hundred and twenty")
            );
            assert_eq!(
                to_word(247, AndBehavior::LastGroup),
                String::from("two-hundred and forty-seven")
            );
            assert_eq!(
                to_word(403, AndBehavior::LastGroup),
                String::from("four-hundred and three")
            );
            assert_eq!(
                to_word(612, AndBehavior::LastGroup),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                to_word(919, AndBehavior::LastGroup),
                String::from("nine-hundred and nineteen")
            );
            assert_eq!(
                to_word(2105, AndBehavior::LastGroup),
                String::from("two thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(200_105, AndBehavior::LastGroup),
                String::from("two-hundred thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(530_175_000, AndBehavior::LastGroup),
                String::from("five-hundred thirty million, one-hundred seventy-five thousand")
            );
            assert_eq!(
                to_word(530_175_999, AndBehavior::LastGroup),
                String::from(
                    "five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_530_175_999, AndBehavior::LastGroup),
                String::from(
                    "four billion, five-hundred thirty million, one-hundred \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_000_175_999, AndBehavior::LastGroup),
                String::from(
                    "four billion, one-hundred \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(14_000_001_019, AndBehavior::LastGroup),
                String::from("fourteen billion, one thousand, nineteen")
            );
            assert_eq!(
                to_word(123_456_789_012_345, AndBehavior::LastGroup),
                String::from(
                    "one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      and forty-five"
                )
            );
            assert_eq!(
                to_word(17_654_123_456_789_012_345, AndBehavior::LastGroup),
                String::from(
                    "seventeen quintillion, six-hundred fifty-four quadrillion, \
                    one-hundred twenty-three trillion, four-hundred fifty-six billion, \
                      seven-hundred eighty-nine million, twelve thousand, three-hundred \
                      and forty-five"
                )
            );
            assert_eq!(
                to_word(u64::MAX, AndBehavior::LastGroup),
                String::from(
                    "eighteen quintillion, four-hundred forty-six quadrillion, \
                 seven-hundred forty-four trillion, seventy-three billion, \
                 seven-hundred nine million, five-hundred fifty-one thousand, \
                 six-hundred and fifteen"
                )
            );
        }
    }
}

pub mod process_input {
    use super::conversion_to_words::AndBehavior;
    use super::to_word;
    use super::COPYRIGHT_INFO;

    #[derive(Clone)]
    enum InputComponent {
        ToConvert(u64),
        Error(String),
        Help,
        AndOption(AndBehavior),
    }

    enum OutputComponent {
        ToConvert(u64, AndBehavior),
        Error(String),
    }

    /// Helper for command-line mode
    ///
    /// # Examples
    ///
    /// ```
    /// use numbers_into_words::Config;
    /// use std::env;
    ///
    /// /* Essentially the entire main function for the command-line program */
    /// let args: Vec<String> = env::args().collect();
    /// let config = Config::parse(args);
    /// println!("{}", config.process());
    /// ```
    ///
    /// ```
    /// use numbers_into_words::Config;
    ///
    /// let args: Vec<String> = vec!["program_name".to_string(), "42".to_string()];
    /// assert_eq!(
    ///     Config::parse(args).process(),
    ///     String::from("42: forty-two\n"));
    /// ```
    pub struct Config {
        output_components: Result<Vec<OutputComponent>, String>,
        help: bool,
        prog_name: String,
    }

    fn help_text(prog_name: &String) -> String {
        format!(
            "numbers_into_words: Converts positive integers to words\n\
                 {}\n\
                 -------------------------------------------------------\n\
                 \n\
                 Usage:\n\
                 $ {} (<number> | help) [<number> | help] ... \n\
                 \n\
                 Example:\n\
                 \n\
                 {}\n\
                 Note: maximum value supported is {} ({})",
            COPYRIGHT_INFO,
            prog_name,
            example_session(
                &["234", "92,582,349", "543_953_459_343", "8"],
                prog_name.as_str()
            ),
            u64::MAX,
            to_word(u64::MAX, AndBehavior::All)
        )
    }

    fn example_session(inputs: &[&str], prog_name: &str) -> String {
        let mut args: Vec<&str> = vec![prog_name];
        for k in inputs {
            args.push(k)
        }
        let command_line = args.join(" ");
        let config = Config::parse(args.iter().map(|x| String::from(*x)).collect());
        let output = config.process();
        format!("$ {}\n{}", command_line, output)
    }

    impl Config {
        /// Parses the command-line arguments and encodes them in a `Config`
        pub fn parse(args: Vec<String>) -> Self {
            let prog_name = args[0].clone();
            if args.len() < 2 {
                return Self {
                    output_components: Err(format!(
                        "No arguments. Try this:\n$ {} help",
                        prog_name
                    )),
                    help: false,
                    prog_name,
                };
            }

            let mut help: bool = false;
            let mut and_behavior: AndBehavior = AndBehavior::All;
            let input_cmpts: Vec<InputComponent> = args[1..]
                .iter()
                .map(|x| InputComponent::parse_single_input(x))
                .collect();
            for k in input_cmpts.clone() {
                match k {
                    InputComponent::Help => {
                        help = true;
                    }
                    InputComponent::AndOption(k) => {
                        and_behavior = k;
                    }
                    _ => {}
                }
            }
            let output_components: Result<Vec<OutputComponent>, String> = Ok(input_cmpts
                .iter()
                .map(|x| match x {
                    InputComponent::ToConvert(k) => {
                        Some(OutputComponent::ToConvert(*k, and_behavior))
                    }
                    InputComponent::Error(k) => Some(OutputComponent::Error(k.clone())),
                    _ => None,
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect());

            Self {
                output_components,
                help,
                prog_name,
            }
        }

        /// Returns the program output appropriate for the command-line arguments used to encode
        /// the `Config`
        pub fn process(&self) -> String {
            match &self.output_components {
                Err(e) => e.clone(),
                Ok(cmpts) => {
                    let mut valid = false;
                    let mut errors = false;

                    let mut valid_vec: Vec<String> = Vec::new();
                    let mut error_vec: Vec<String> = Vec::new();

                    for c in cmpts {
                        match c {
                            OutputComponent::ToConvert(k, and_behavior) => {
                                valid_vec.push(format!("{}: {}", k, to_word(*k, *and_behavior)));
                                valid = true;
                            }
                            OutputComponent::Error(e) => {
                                error_vec.push(e.clone());
                                errors = true;
                            }
                        }
                    }
                    let mut valid_conversions = String::new();
                    if !valid_vec.is_empty() && self.help {
                        valid_conversions.push_str("\n---\n\n");
                    }
                    for k in valid_vec {
                        valid_conversions.push_str(k.as_str());
                        valid_conversions.push('\n');
                    }
                    if valid && errors {
                        valid_conversions.push('\n');
                    }

                    let errors = if !error_vec.is_empty() {
                        format!("Errors\n-----\n{}", error_vec.join("\n"))
                    } else {
                        String::from("")
                    };

                    format!(
                        "{}{}{}",
                        if self.help {
                            help_text(&self.prog_name)
                        } else {
                            "".to_string()
                        },
                        valid_conversions,
                        errors
                    )
                }
            }
        }
    }

    impl InputComponent {
        fn parse_single_input(text: &str) -> Self {
            let cleaned = text.to_lowercase();
            if cleaned.len() > 2 && &cleaned[..2] == "--" {
                if &cleaned[2..] == "help" {
                    Self::Help
                } else if &cleaned[2..6] == "and=" {
                    match &cleaned[6..] {
                        "none" => Self::AndOption(AndBehavior::None),
                        "last" => Self::AndOption(AndBehavior::LastGroup),
                        "below1k" => Self::AndOption(AndBehavior::OnlyUnderThousand),
                        "all" => Self::AndOption(AndBehavior::All),
                        k => Self::Error(format!("Invalid \"and\" option: {}", k)),
                    }
                } else {
                    Self::Error(format!("Invalid option {}", cleaned))
                }
            } else {
                let n_text = cleaned
                    .chars()
                    .filter(|x| x.is_ascii_digit())
                    .collect::<String>();
                if n_text.is_empty() {
                    Self::Error(format!("Invalid input: {}", text))
                } else {
                    match n_text.parse::<u64>() {
                        Ok(x) => Self::ToConvert(x),
                        Err(_) => Self::Error(format!("Too big: {}", text)),
                    }
                }
            }
        }
    }
}
