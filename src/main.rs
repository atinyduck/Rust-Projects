//
// To Do List
// By Jake Morgan 21-10-2024
// Produced using Rust
// This program will allow users to manage and keep tasks in a simple console based UI.
//

use std::io::{self, BufRead};
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    //
    // Loop through menu inputs to keep program running 
    //

    loop {
        print!("{esc}c", esc = 27 as char); // Clear Screen

        // Menu Options
        println!("To-Do List:\n");
        println!("1. View tasks");
        println!("2. Add tasks");
        println!("3. Mark tasks as done");
        println!("4. Quit\n\nChoice:");
        
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
    //
    // Allow the user to view all their saved tasks
    //

    print!("{esc}c", esc = 27 as char); // Clear Screen

    println!("\n* * * View Tasks * * *");

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

    // Prevent the loop from clearing before the user is done reading
    let mut end: String = String::new();
    println!("\nPress enter to continue...");
    io::stdin()
        .read_line(&mut end)
        .expect("CONTINUE");

}

fn add_tasks(){
    //
    // Allow the user to add a task to their file.
    //
    // Create the file if it does not yet exist.
    //

    print!("{esc}c", esc = 27 as char); // Clear Screen

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
        // Output each file in a numbered list
        Ok(mut file) => {
            // Write the new task to the file 
            writeln!(file, "{}", task.trim()).expect("\n* Failed to write to file *\n");
            println!("\n * Task added successfully *\n");
        } 
        Err(_) => println!("\n* Error reading tasks *\n"),
    }
}

fn mark_tasks_done() {
    //
    // Allow the user to delete tasks from their list
    //

    print!("{esc}c", esc = 27 as char); // Clear Screen

    println!("\n* * * Remove Tasks * * *\n");

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt");

    // Create a vectore to store all current tasks
    let mut tasks = Vec::new();

    match file {
        // Output the tasks in a numbered list and store them in the vector
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for (index, line) in reader.lines().enumerate() {
                match line {
                    Ok(task) => {
                        println!("{}. {}", index + 1, task);
                        tasks.push(task);
                    }
                    Err(_) => println!("\n* Error reading tasks *\n")
                }
            }
        }
        Err(_) => println!("\n* No tasks Found *\n")
    }

    // Ask for the users choice
    println!("Enter the number of the task to mark as done:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let task_num: usize = input.trim()
        .parse()
        .expect("Invalid number");

    // Remove the selected task
    if task_num > 0 && task_num <= tasks.len() {
        tasks.remove(task_num - 1);  
        println!("Task marked as done!");
    } else {
        println!("Invalid task number.");
    }

    // Save the changes 
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)  // Clear the file before writing the updated tasks
        .open("tasks.txt")
        .expect("Failed to open file");

    // Write the changes to the actual file
    for task in tasks {
        writeln!(file, "{}", task).expect("Failed to write to file");
    }
}