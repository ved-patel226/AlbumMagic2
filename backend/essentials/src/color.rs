use colored::Colorize;

pub fn print_success(message: &str) {
    println!("{}", message.green().bold());
}

pub fn print_error(message: &str) {
    println!("{}", message.red().bold());
}

pub fn print_warning(message: &str) {
    println!("{}", message.yellow().bold());
}

pub fn print_info(message: &str) {
    println!("{}", message.blue());
}

pub fn print_color(message: &str, color: colored::Color) {
    println!("{}", message.color(color).bold());
}
