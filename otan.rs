use std::io::{self, BufRead};

fn otan() {
    println!("Ecrivez le texte à transcrire en OTAN : ");
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut otan_input = String::new();
    handle.read_line(&mut otan_input).expect("Erreur lors de la lecture de l'entrée");

    let otan_input = otan_input.trim().to_uppercase();

    if otan_input.chars().any(|c| c.is_ascii_digit()) {
        println!("\nVeuillez ne pas mettre de nombres.");
        return;
    }

    let mut translation = String::with_capacity(otan_input.len() * 6); // Pré-allouer suffisamment de capacité

    for c in otan_input.chars() {
        if let Some(otan_code) = lookup_otan(c) {
            translation.push_str(otan_code);
            translation.push(' ');
        }
    }

    println!("\n{}\n", translation.trim());
}

fn lookup_otan(c: char) -> Option<&'static str> {
    match c {
        ' ' => Some(" "),
        'A'..='Z' => {
            const OTAN_TABLE: [&str;26] = [
                "Alpha", "Bravo", "Charlie", "Delta", "Echo", "Foxtrot", "Golf", "Hotel",
                "India", "Juliet", "Kilo", "Lima", "Mike", "November", "Oscar", "Papa",
                "Quebec", "Romeo", "Sierra", "Tango", "Uniform", "Victor", "Whiskey",
                "Xray", "Yankee", "Zulu",
            ];
            Some(OTAN_TABLE[(c as u8 - b'A') as usize])
        }
        _ => None,
    }
}

fn main() {
    otan();
}
