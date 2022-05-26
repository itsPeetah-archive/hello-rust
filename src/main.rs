use ferris_says::say;
use std::io::{stdout, BufWriter, Write};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    println!("Hello Rust!");

    let prompt = String::from("Type in yout message: ");
    let mut input = String::new();
    get_input(prompt, &mut input);

    let char_count = input.chars().count() as f32;

    // Ternary operator <3
    let max_width = if char_count > 10.0 {
        0.67 * char_count
    } else {
        char_count
    };

    say(input.as_bytes(), max_width as usize, &mut writer).unwrap();
}

fn get_input(prompt: String, input: &mut String) {
    print!("{}", prompt);

    // Flush stdout: stdout is buffered and
    // since print! does not print a "\n" I need to
    // manually trigger a flush
    std::io::stdout()
        .flush()
        .expect("[getInput] flush failed...");

    match std::io::stdin().read_line(input) {
        Ok(_) => {}
        Err(err) => println!("Error while parsing input: {}", err),
    }
}
