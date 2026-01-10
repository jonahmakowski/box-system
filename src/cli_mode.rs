use super::{CODES_FOLDER, DB_FILE, database};
use colored::*;
use simple_lib::input;
use std::{env, fs};

pub fn run() {
    fs::create_dir_all(CODES_FOLDER).expect("Failed to create codes folder");

    let mut database_contents = database::read_database(DB_FILE).expect("Failed to load DB");

    let mut args = env::args();

    args.next();

    match args.next() {
        Some(arg) => match arg {
            arg if arg == "new" => new(&mut database_contents),
            arg if arg == "list" => list_all(&database_contents),
            arg if arg == "edit" => edit(&mut database_contents),
            _ => panic!("Invalid arguemnt"),
        },
        None => panic!("Failed to parse argument"),
    }

    database::save_database(database_contents, DB_FILE).expect("Failed to write DB");
}

fn new(db_data: &mut Vec<database::BoxData>) {
    println!("What's inside this box? Input below:");
    let box_contents = input::get_string_input().expect("Failed to get input");

    let new_data = database::BoxData::new(box_contents.trim().to_string());

    println!("Qr code at: {}", new_data.get_pdf_path());

    db_data.push(new_data);
}

fn list_all(db_data: &Vec<database::BoxData>) {
    println!(
        "{:<50}{:<40}{}",
        "Box Contents".bold(),
        "UUID".bold(),
        "PDF file".bold()
    );
    for data in db_data {
        println!(
            "{:<50}{:<40}{}",
            data.box_contents,
            data.uuid(),
            data.get_pdf_path()
        )
    }
}

fn edit(db_data: &mut Vec<database::BoxData>) {
    println!("Enter the UUID of the box you want to edit:");
    let box_uuid = input::get_string_input().expect("Failed to get input");

    let box_data = match database::find_by_uuid(db_data, &box_uuid.trim()) {
        Some(d) => d,
        None => {
            println!("That's not a valid uuid. Try running `list` to see all the UUIDs");
            return;
        }
    };

    println!("The old contents: {}", box_data.box_contents);
    println!("What would you like the new contents to be?");
    box_data.box_contents = input::get_string_input()
        .expect("Failed to get input")
        .trim()
        .to_string();
}
