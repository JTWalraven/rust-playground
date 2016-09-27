extern crate colored;

use colored::*;

fn main() {
    println!("{} {}{}", 
             "Hello".italic().green(), 
             "world".blue().bold(), 
             "!!!");
    println!("{}", "There should be some colors.".on_blue().red());
}
