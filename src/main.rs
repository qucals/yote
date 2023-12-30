fn main() {
    let mut notes = Vec::new();

    loop {
        print_menu();
        let command = read_input();

        match command.trim() {
            "show" => show_notes(&notes),
            "add" => add_note(&mut notes),
            _ => break,
        }
    }
}

fn print_menu() {
    println!();
    println!();
    println!("*** PROGRAM MENU ***");
    println!("Enter command:");
    println!("'show' - show all notes");
    println!("'add' - add new note");
    println!("other - exit");
    println!("other - exit");
}

fn read_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn show_notes(notes: &Vec<String>) {
    println!();

    for note in notes {
        println!("{}", note)
    }
}

fn add_note(notes: &mut Vec<String>) {
    println!();
    println!("Enter note:");

    let input = read_input();
    let note = input.trim().to_string();

    notes.push(note);
}
