//
// To Do List
// By Jake Morgan 21-10-2024
//
// This program will allow users to manage and keep tasks in a simple console based UI.
//

use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    loop {
        print!("{esc}c", esc = 27 as char);

        // Menu Options
        println!("To-Do List:\n");
        println!("1. View tasks");
        println!("2. Add tasks");
        println!("3. Mark tasks as done");
        println!("4. Quit");
        
        //Get user input
        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("\n* Failed to read input. *\n");

        match choice.trim() {
            "1" => view_tasks(),
            "2" => add_tasks(),
            "3" => mark_tasks_done(),
            "4" => break,
            _ => println!("\n* Invalid Option *\n"),
        }
    }
}

fn view_tasks(){
    print!("{esc}c", esc = 27 as char);

    // Reads the contents of the file
    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt");

    match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            // Output each task in the file.
            for (index, line) in reader.lines().enumerate() {
                match line {
                    Ok(task) => println!("{}. {}", index + 1, task),
                    Err(_) => println!("Error reading task."),
                }
            }
        }
        Err(_) => println!("\n* No Tasks Found *\n")
    }
    println!("\n");

}

fn add_tasks(){
    print!("{esc}c", esc = 27 as char);

    let mut task: String = String::new();
    println!("\nEnter the task you would like to add:");

    // Get user input.
    io::stdin()
        .read_line(&mut task)
        .expect("\n* Failed to read input *\n");

    let file = OpenOptions::new()
        .append(true)  // Prevents overwriting tasks
        .create(true)  // Create the file if it doesn't exist
        .open("tasks.txt");

    match file {
        Ok(mut file) => {
            writeln!(file, "{}", task.trim()).expect("\n* Failed to write to file *\n");
            println!("\n * Task added successfully *\n");
        } 
        Err(_) => println!("\ns* Error reading tasks *\n"),
    }
}

fn mark_tasks_done(){

}