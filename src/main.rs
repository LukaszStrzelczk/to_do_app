mod task;
use std::{
    io, thread,
    time::{self, Duration},
};
use task::to_do_list::ToDoList;
fn main() {
    println!("Hello, world!");
    let mut input_buffer = String::new();
    let mut list = ToDoList::new();
    loop {
        input_buffer.clear();
        cls();
        list.show();
        println!("1. Add new Task");
        println!("2. Delete Task");
        println!("3. Clear all");
        println!("4. Clear done");
        println!("5. Mark as done");
        println!("6. Save");
        println!("7. Load");
        println!("0. Exit");
        println!("==============================");
        println!("What do you want to do: ");
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("error while reading line");
        let choice = match input_buffer.trim().parse::<usize>() {
            Ok(r) => r,
            Err(_) => {
                println!("Invalid Input it must be a number from 0 to 5");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
        };
        match choice {
            0 => break,
            1 => add_new_task(&mut list),
            2 => delete_task(&mut list),
            3 => list.clear_all(),
            4 => list.clear_done(),
            5 => mark_as_done(&mut list),
            6 => save_to_file(&list),
            7 => load_from_file(&mut list),
            _ => {
                println!("Invalid Input it must be a number from 0 to 6");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
        }
    }
}

fn cls() {
    println!("{}[2J", 27 as char); // clear the terminal
}
/// Prompts the user for input to create a new task and adds it to the provided ToDoList.
/// # Arguments
/// * `list` - A mutable reference to the `ToDoList` where the new task will be added.
fn add_new_task(list: &mut ToDoList) {
    cls();
    let mut title = String::new();
    println!("Enter Task title: ");
    io::stdin()
        .read_line(&mut title)
        .expect("error while reading line");
    let mut description = String::new();
    println!("Enter Task description: ");
    io::stdin()
        .read_line(&mut description)
        .expect("error while reading a line");
    list.add_task(&title, &description);
}
/// Prompts the user for input to delete specific task and removes it from `tasks` list
/// # Arguments
/// * `list` - A mutable reference to the `ToDoList` from which selcted task will be removed
fn delete_task(list: &mut ToDoList) {
    cls();
    list.show();
    let mut idx: usize;
    loop {
        let mut index = String::new();
        println!("Enter index of a Task you want to remove (or 0 to Exit): ");
        io::stdin()
            .read_line(&mut index)
            .expect("Unable to read a line");
        match index.trim().parse::<usize>() {
            Ok(id) => idx = id,
            Err(_) => {
                println!("your input is wrong try again");
                continue;
            }
        };
        if idx == 0 {
            break;
        } //exit if user type in "0"
        if let Err(e) = list.remove_task(idx - 1) {
            println!("{e}.");
            continue;
        }
        break;
    }
}
/// Prompts the user for input to complete specific task and removes it from `tasks` list and adds it to `done` list
/// # Arguments
/// * `list` - A mutable reference to the `ToDoList` 
fn mark_as_done(list: &mut ToDoList) {
    cls();
    list.show();
    let mut idx: usize;
    loop {
        let mut index = String::new();
        println!("Enter index of a Task you want to complete (or 0 to Exit): ");
        io::stdin()
            .read_line(&mut index)
            .expect("Unable to read a line");
        match index.trim().parse::<usize>() {
            Ok(id) => idx = id,
            Err(_) => {
                println!("your input is wrong try again");
                continue;
            }
        };
        if idx == 0 {
            break;
        } //exit if user type in "0"
        if let Err(e) = list.complete_task(idx - 1) {
            println!("{e}.");
            continue;
        }
        break;
    }
}

/// Enables user to save current `ToDoList` state to a txt file
/// # Arguments
/// * `list` - A reference to the `ToDoList` which will be saved to a file
fn save_to_file(list: &ToDoList) {
    let mut buf = String::new();
    println!("Enter file name:");
    io::stdin()
        .read_line(&mut buf)
        .expect("unable to read a line");
    match list.save_to_file(buf.trim()) {
        Err(e) => {
            println!("saving to file has failed. Error: {e}");
            thread::sleep(Duration::from_secs(4));
        }
        Ok(_) => {
            println!("Saved succesfully!!");
            thread::sleep(Duration::from_millis(1500));
        }
    }
}

/// Enables user to load `ToDoList` state from a txt file
/// # Arguments
/// * `list` - A mutable reference to the `ToDoList` 
fn load_from_file(list: &mut ToDoList){
    todo!();
}