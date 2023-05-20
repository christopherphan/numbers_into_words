/* src/main.rs
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

use numbers_into_words::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(args);
    println!("{}", config.process());
}
