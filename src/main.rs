/* Lots of match statement examples
 *
 * Christopher Phan
 * 2023-W20
 *
 * */

use number_words::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(args);
    println!("{}", config.process());
}
