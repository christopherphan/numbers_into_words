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

const COPYRIGHT_INFO: &str = "\
         Copyright \u{00a9} 2023 Christopher Phan\n\
         https://chrisphan.com/\n\
         Licensed under MIT or APACHE 2.0";

pub use conversion_to_words::to_word;
pub use process_input::Config;

pub mod conversion_to_words {
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

    fn under_1000(x: u64) -> Result<String, &'static str> {
        match x {
            0..=99 => under_100(x),
            100..=900 if x % 100 == 0 => Ok(format!(
                "{}-hundred",
                single_digit(x / 100).expect("under 10")
            )),
            x if x < 1000 => Ok(format!(
                "{} and {}",
                under_1000(x - (x % 100)).expect("under 1000"),
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
    /// use numbers_into_words::to_word;
    ///
    /// // US population according to 2020 census
    /// // https://www2.census.gov/library/publications/decennial/2020/census-briefs/c2020br-01.pdf
    ///
    /// assert_eq!(
    ///     to_word(330_759_736),
    ///     String::from(
    ///         "three-hundred and thirty million, \
    ///         seven-hundred and fifty-nine thousand, \
    ///         seven-hundred and thirty-six"
    ///     )
    /// );
    ///
    /// assert_eq!(to_word(0), "zero".to_string());
    /// ```
    pub fn to_word(x: u64) -> String {
        if x == 0 {
            single_digit(0).expect("under 10")
        } else {
            (0..7)
                .map(|y| ((x / (10_u64).pow(3 * (6 - y as u32))) % 1000, 6 - y))
                .filter(|(a, _)| *a != 0_u64)
                .map(|(a, b)| {
                    format!(
                        "{}{}",
                        under_1000(a).expect("under 1000"),
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
            assert_eq!(under_1000(3).unwrap(), String::from("three"));
            assert_eq!(under_1000(12).unwrap(), String::from("twelve"));
            assert_eq!(under_1000(19).unwrap(), String::from("nineteen"));
            assert_eq!(under_1000(20).unwrap(), String::from("twenty"));
            assert_eq!(under_1000(47).unwrap(), String::from("forty-seven"));
            assert_eq!(
                under_1000(120).unwrap(),
                String::from("one-hundred and twenty")
            );
            assert_eq!(
                under_1000(247).unwrap(),
                String::from("two-hundred and forty-seven")
            );
            assert_eq!(
                under_1000(403).unwrap(),
                String::from("four-hundred and three")
            );
            assert_eq!(
                under_1000(612).unwrap(),
                String::from("six-hundred and twelve")
            );
            assert_eq!(
                under_1000(919).unwrap(),
                String::from("nine-hundred and nineteen")
            );
            assert!(under_1000(2105).is_err());
        }

        #[test]
        fn test_to_word() {
            assert_eq!(to_word(0), String::from("zero"));
            assert_eq!(to_word(3), String::from("three"));
            assert_eq!(to_word(12), String::from("twelve"));
            assert_eq!(to_word(19), String::from("nineteen"));
            assert_eq!(to_word(20), String::from("twenty"));
            assert_eq!(to_word(47), String::from("forty-seven"));
            assert_eq!(to_word(120), String::from("one-hundred and twenty"));
            assert_eq!(to_word(247), String::from("two-hundred and forty-seven"));
            assert_eq!(to_word(403), String::from("four-hundred and three"));
            assert_eq!(to_word(612), String::from("six-hundred and twelve"));
            assert_eq!(to_word(919), String::from("nine-hundred and nineteen"));
            assert_eq!(
                to_word(2105),
                String::from("two thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(200_105),
                String::from("two-hundred thousand, one-hundred and five")
            );
            assert_eq!(
                to_word(530_175_000),
                String::from(
                    "five-hundred and thirty million, one-hundred and seventy-five thousand"
                )
            );
            assert_eq!(
                to_word(530_175_999),
                String::from(
                    "five-hundred and thirty million, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_530_175_999),
                String::from(
                    "four billion, five-hundred and thirty million, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(4_000_175_999),
                String::from(
                    "four billion, one-hundred and \
                         seventy-five thousand, nine-hundred and ninety-nine"
                )
            );
            assert_eq!(
                to_word(14_000_001_019),
                String::from("fourteen billion, one thousand, nineteen")
            );
            assert_eq!(
                to_word(123_456_789_012_345),
                String::from(
                    "one-hundred and twenty-three trillion, four-hundred and fifty-six billion, \
                      seven-hundred and eighty-nine million, twelve thousand, three-hundred and \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(17_654_123_456_789_012_345),
                String::from(
                    "seventeen quintillion, six-hundred and fifty-four quadrillion, \
                    one-hundred and twenty-three trillion, four-hundred and fifty-six billion, \
                      seven-hundred and eighty-nine million, twelve thousand, three-hundred and \
                      forty-five"
                )
            );
            assert_eq!(
                to_word(u64::MAX),
                String::from(
                    "eighteen quintillion, four-hundred and forty-six quadrillion, \
                 seven-hundred and forty-four trillion, seventy-three billion, \
                 seven-hundred and nine million, five-hundred and fifty-one thousand, \
                 six-hundred and fifteen"
                )
            );
        }
    }
}

pub mod process_input {
    use super::to_word;
    use super::COPYRIGHT_INFO;

    enum InputComponent {
        ToConvert(u64),
        Help,
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
        components: Result<Vec<InputComponent>, String>,
        prog_name: String,
    }

    fn help_text(help_triggered: bool, prog_name: &String) -> String {
        if help_triggered {
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
                to_word(u64::MAX)
            )
        } else {
            String::new()
        }
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
            let components = if args.len() < 2 {
                Err(format!("No arguments. Try this:\n$ {} help", prog_name))
            } else {
                Ok(args[1..]
                    .iter()
                    .map(|x| InputComponent::parse_single_input(x))
                    .collect())
            };
            Self {
                components,
                prog_name,
            }
        }

        /// Returns the program output appropriate for the command-line arguments used to encode
        /// the `Config`
        pub fn process(&self) -> String {
            match &self.components {
                Err(e) => e.clone(),
                Ok(cmpts) => {
                    let mut help_triggered = false;
                    let mut valid = false;
                    let mut errors = false;

                    let mut valid_vec: Vec<String> = Vec::new();
                    let mut error_vec: Vec<String> = Vec::new();

                    for c in cmpts {
                        match c {
                            InputComponent::ToConvert(k) => {
                                valid_vec.push(format!("{}: {}", k, to_word(*k)));
                                valid = true;
                            }
                            InputComponent::Help => {
                                help_triggered = true;
                            }
                            InputComponent::Error(e) => {
                                error_vec.push(e.clone());
                                errors = true;
                            }
                        }
                    }
                    let mut valid_conversions = String::new();
                    if !valid_vec.is_empty() && help_triggered {
                        valid_conversions.push_str("\n---\n\n");
                    }
                    for k in valid_vec {
                        valid_conversions.push_str(k.as_str());
                        valid_conversions.push('\n');
                    }
                    if valid && errors {
                        valid_conversions.push('\n');
                    }

                    let help = help_text(help_triggered, &self.prog_name);

                    let errors = if !error_vec.is_empty() {
                        format!("Errors\n-----\n{}", error_vec.join("\n"))
                    } else {
                        String::from("")
                    };

                    format!("{}{}{}", help, valid_conversions, errors)
                }
            }
        }
    }

    impl InputComponent {
        fn parse_single_input(text: &str) -> Self {
            let cleaned = (*text)
                .to_lowercase()
                .chars()
                .filter(|x| x.is_ascii_digit() || x == &'h' || x == &'e' || x == &'l' || x == &'p')
                .collect::<String>();
            match cleaned.as_str() {
                "help" => Self::Help,
                k => {
                    let n_text = k.chars().filter(|x| x.is_ascii_digit()).collect::<String>();
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
}
