pub mod to_do {
    use core::fmt;
    use std::fmt::Display;

    /// A `Task` represents a single task in the to-do list.
    #[derive(Debug, Default, Clone)]
    pub struct Task {
        title: String,
        description: String,
    }

    impl Task {
        /// Creates new `Task` with given values for `title` and `description`
        pub fn new(title: &str, description: &str) -> Self {
            Task {
                title: title.to_owned(),
                description: description.to_owned(),
            }
        }
        #[allow(unused)]
        /// returns `title`
        pub fn title(&self) -> &str {
            &self.title
        }
        #[allow(unused)]
        /// returns `description`
        pub fn description(&self) -> &str {
            &self.description
        }
    }

    impl Display for Task {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} \nDescription: \n{}", self.title, self.description)
        }
    }

    impl PartialEq for Task {
        fn eq(&self, other: &Self) -> bool {
            self.title == other.title
        }
    }
}

pub mod to_do_list {
    use std::{
        fs::{create_dir, File},
        io::{BufRead, Write},
        path::Path,
    };
    #[allow(unused)]
    enum Section {
        Tasks,
        Done,
    }
    use super::to_do::Task;
    /// A `ToDoList` represents a list of `[Task]`, separated into "pending" and "done" categories.
    #[derive(Debug, Default, Clone)]
    pub struct ToDoList {
        tasks: Vec<Task>,
        done: Vec<Task>,
    }

    impl ToDoList {
        /// Creates a new empty to-do list.
        pub fn new() -> Self {
            ToDoList {
                tasks: Vec::new(),
                done: Vec::new(),
            }
        }
        /// Creates a new to-do list with already existing vecotrs for `tasks` ad `done`
        pub fn new_from_exisitng(tasks: Vec<Task>, done: Vec<Task>) -> Self {
            ToDoList { tasks, done }
        }
        /// Add new `[Task]` to pending list with `title` and `description`
        pub fn add_task(&mut self, title: &str, description: &str) {
            let task = Task::new(title, description);
            self.tasks.push(task)
        }
        /// Removes `Task` from `pending` and moves it to `done` vector
        pub fn complete_task(&mut self, id: usize) -> Result<(), &str> {
            if self.tasks.is_empty() {
                return Err("List is empty");
            } else if id >= self.tasks.len() {
                return Err("There is no Task with this ID");
            }
            self.done.push(self.tasks.remove(id));
            Ok(())
        }
        /// Removes `Task` from `pending` and deletes it
        pub fn remove_task(&mut self, id: usize) -> Result<(), &str> {
            if self.tasks.is_empty() {
                return Err("List is empty");
            } else if id >= self.tasks.len() {
                return Err("There is no Task with this ID");
            }
            self.tasks.remove(id);
            Ok(())
        }
        /// Clears both `pending` and `done` lists
        pub fn clear_all(&mut self) {
            self.tasks.clear();
            self.done.clear();
        }
        /// Clears `done` list
        pub fn clear_done(&mut self) {
            self.done.clear();
        }
        // Shows contents of both `pending` and `done` lists
        pub fn show(&self) {
            println!("Tasks: ");
            if self.tasks.is_empty() {
                println!("Empty")
            } else {
                for (i, task) in self.tasks.iter().enumerate() {
                    println!("{}.\n{}", i + 1, task);
                }
            }
            println!("Done: ");
            if self.done.is_empty() {
                println!("Empty")
            } else {
                for (i, task) in self.done.iter().enumerate() {
                    println!("{}.\n{}", i + 1, task);
                }
            }
        }
        /// Enables saving current `ToDoList` state to a txt file
        pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
            if !Path::new("saves").exists() {
                create_dir("saves")?;
            };
            let filename = {
                if filename.is_empty() {
                    String::from("save1")
                } else {
                    String::from(filename)
                }
            };
            let full_filename = format!("{}.txt", filename);
            let file_path = Path::new("saves").join(full_filename);
            let mut file = File::create(file_path)?;
            writeln!(file, "[Tasks]")?;
            for i in self.tasks.iter() {
                writeln!(file, "Title: {}", i.title())?;
                writeln!(file, "Description: {}", i.description())?;
            }
            writeln!(file, "[Done]")?;
            for i in self.done.iter() {
                writeln!(file, "Title: {}", i.title())?;
                writeln!(file, "Description: {}", i.description())?;
            }
            Ok(())
        }
        /// Enable reading from a file
        /// # Arguments
        /// * `file_path` - A `&str` that represents a path to a file
        pub fn read_from_file(&mut self, file_path: &str) -> Result<(), std::io::Error> {
            let file = File::open(file_path)?;
            let reader = std::io::BufReader::new(file);
            let mut section: Option<Section> = None;
            let mut title=String::new();
            let mut description;
            for line in reader.lines() {
                let line = line?; //error propagation
                if line.is_empty() {
                    continue;
                }
                if line == "[Tasks]" {
                    section = Some(Section::Tasks);
                    continue;
                } else if line == "[Done]" {
                    section = Some(Section::Done);
                    continue;
                }
                if line.starts_with("Title: ") {
                    title = line.trim_start_matches("Title: ").to_owned();
                    continue;
                };
                if line.starts_with("Description: ") {
                    description = line.trim_start_matches("Description: ");
                    match section {
                        Some(Section::Tasks) => {
                            let task = Task::new(&title, description);
                            self.tasks.push(task);
                        }
                        Some(Section::Done) => {
                            let task = Task::new(&title, description);
                            self.done.push(task);
                        }
                        None => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "File is corrupted",
                            ));
                        }
                        
                    }
                };

            }
            Ok(())
        }

        #[allow(unused)]
        /// returns `tasks`
        pub fn taks(&self) -> Vec<Task> {
            self.tasks.clone()
        }
        #[allow(unused)]
        /// returns `done`
        pub fn done(&self) -> Vec<Task> {
            self.done.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{to_do::*, to_do_list::*};
    #[test]
    fn test_task_new() {
        let list = ToDoList::new();
        assert!(list.done().is_empty());
        assert!(list.taks().is_empty());
    }

    #[test]
    fn add_task_test() {
        let mut list = ToDoList::new();

        list.add_task("Cooking", "Prepare meal for tomorrow");
        let len = list.taks().len();

        assert_eq!(list.taks()[len - 1].title(), "Cooking");
        assert_eq!(
            list.taks()[len - 1].description(),
            "Prepare meal for tomorrow"
        );
    }
    #[test]
    fn complete_task_test() {
        let mut list = ToDoList::new();

        list.add_task("Cooking", "Prepare meal for tomorrow");
        #[allow(unused_must_use)]
        list.complete_task(0);

        assert!(list.taks().is_empty());
        assert!(!list.done().is_empty());
    }
    #[test]
    fn remove_task_test() {
        let mut list = ToDoList::new();

        list.add_task("Cooking", "Prepare meal for tomorrow");
        list.add_task("Cooking2", "Prepare meal for the day after tomorrow");

        assert_eq!(list.taks().len(), 2);
        #[allow(unused_must_use)]
        list.remove_task(0);
        assert_eq!(list.taks().len(), 1);

        let check = Task::new("Cooking", "unrelevenat for the check");
        assert!(!list.taks().contains(&check));
    }
    #[test]
    fn remove_task_test_fail() {
        let mut list = ToDoList::new();

        list.add_task("Cooking", "Prepare meal for tomorrow");
        list.add_task("Cooking2", "Prepare meal for the day after tomorrow");

        assert_eq!(list.taks().len(), 2);

        let _ = list.remove_task(1);
        assert_eq!(list.taks().len(), 1);

        let check = Task::new("Cooking", "unrelevenat for the check");
        assert!(list.taks().contains(&check));
    }
    #[test]
    fn save_to_file_test() -> std::io::Result<()> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        // Prepare test data
        let tasks = vec![
            Task::new("Learn Rust", "Complete the Rust book and exercises."),
            Task::new("Grocery Shopping", "Buy milk, eggs, and bread."),
        ];
        let done = vec![Task::new("Morning Run", "Ran 5km in the park.")];
        let tdl = ToDoList::new_from_exisitng(tasks, done);
        tdl.save_to_file("test")?;
        // Read the file back and verify its contents

        let test_file_path = std::path::Path::new("saves/test.txt");
        let file = File::open(test_file_path)?;
        let reader = BufReader::new(file);

        let expected_content = vec![
            "[Tasks]",
            "Title: Learn Rust",
            "Description: Complete the Rust book and exercises.",
            "Title: Grocery Shopping",
            "Description: Buy milk, eggs, and bread.",
            "[Done]",
            "Title: Morning Run",
            "Description: Ran 5km in the park.",
        ];

        for (line, expected) in reader.lines().zip(expected_content) {
            assert_eq!(line?, expected);
        }

        // Cleanup: Remove the test file
        std::fs::remove_file(test_file_path)?;
        Ok(())
    }
   #[test]
    fn test_read_from_file() {
        let tasks = vec![
            Task::new("Learn Rust", "Complete the Rust book and exercises."),
            Task::new("Grocery Shopping", "Buy milk, eggs, and bread."),
        ];
        let done = vec![Task::new("Morning Run", "Ran 5km in the park.")];
        let list = ToDoList::new_from_exisitng(tasks.clone(), done.clone());
        list.save_to_file("test").unwrap();
        let mut list2 = ToDoList::new();
        list2.read_from_file("saves/test.txt").unwrap();
        assert_eq!(list2.taks(), tasks);
        assert_eq!(list2.done(), done);
    }
}
