use colored::Colorize;

pub struct Notifier {}

impl Notifier {
    pub fn display_error(error_title: String, error_message: String) {
        let headline = "🔴 ERROR:";
        println!("{} {} - {}", headline.red().bold(), error_title.red().bold(), error_message.red());
    }

    pub fn display_success(message: String) {
        let headline = "🟩 OK:";
        println!("{} {}", headline.green().bold(), message.green());
    }

    pub fn display_info(message: String) {
        let headline = "🔹 INFO:";
        println!("{} {}", headline.blue().bold(), message);
    }

    pub fn display_warning() {}
}