use std::io::{self, Write};

enum Choice {
    Quit,
    Continue,
}

fn get_choice() -> Choice {
    print!("To continue press Y/y and to quit press N/n: ");
    io::stdout().flush().unwrap();

    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();

    match user_choice.trim() {
        "Y" | "y" => Choice::Continue,
        "N" | "n" => Choice::Quit,
        _ => {
            println!("Invalid choice, continuing...");
            Choice::Continue
        }
    }
}

fn add_new_line(final_lines: &mut String) {
    let mut line_count = 1;
    println!("Enter the lines (enter _#q to quit)");
    loop {
        print!("{} >> ", line_count);
        io::stdout().flush().unwrap();

        let mut new_line = String::new();
        io::stdin().read_line(&mut new_line).unwrap();

        match new_line.trim() {
            "" => println!("Empty line detected, please enter a valid line"),
            "_#q" => break,
            _ => {
                final_lines.push_str(&new_line);
                line_count += 1;
            }
        }
    }
}

fn main() {
    let mut final_input = String::new();

    loop {
        add_new_line(&mut final_input);

        if let Choice::Quit = get_choice() {
            break;
        }
    }

    println!("You entered these lines \n{}", final_input)
}
