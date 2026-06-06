use crate::input;

struct Task {
    task: String,
    done: bool,
    priority: Priority,
}

enum Priority {
    Low,
    Medium,
    High,
}

fn add_task(tasks: &mut Vec<Task>) {
    let task_to_add: String = input("Enter task to add: ");

    let priority_level: Priority = loop {
        let priority: String = input("Enter priority level(Low, Medium, High): ");

        match priority.as_str() {
            "low" => break Priority::Low,
            "medium" => break Priority::Medium,
            "high" => break Priority::High,
            _ => println!("{priority} is not a valid priority level."),
        }
    };

    tasks.push(Task {
        task: task_to_add,
        done: false,
        priority: priority_level,
    });
}

fn remove_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }

    let task_to_remove = loop {
        let index: String = input(&format!(
            "Enter task to index to remove(1-{}): ",
            tasks.len()
        ));
        let index: usize = match index.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Given input is not a valid index!");
                continue;
            }
        };

        if index == 0 || index > tasks.len() {
            println!("Invalid task index!");
            continue;
        } else {
            break index;
        }
    };

    tasks.remove(task_to_remove - 1);
}

fn mark_task_done(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }

    let task_to_mark_done = loop {
        let index: String = input(&format!(
            "Enter task's index to mark done(1-{}): ",
            tasks.len()
        ));
        let index: usize = match index.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: Given input is not a valid index!");
                continue;
            }
        };

        if index == 0 || index > tasks.len() {
            println!("Invalid task index!");
            continue;
        } else {
            break index;
        }
    };

    tasks[task_to_mark_done - 1].done = true;
}

fn view_task(tasks: &Vec<Task>) {
    println!("----- Tasks -----");

    if tasks.is_empty() {
        println!("No tasks added yet!");
        return;
    }

    for (index, task) in tasks.iter().enumerate() {
        println!(
            "Index: {}. Task: {}. Priority: {}. Completed: {}",
            index + 1,
            task.task,
            match &task.priority {
                Priority::Low => "Low", 
                Priority::Medium => "Medium",
                Priority::High => "High",
            },
            (if task.done { "✔️" } else { "✖️" })
        )
    }

    println!("-----------------");
}

pub fn run() {
    println!("Welcome to the Todo List");

    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("----- Commands -----");
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. Mark Task Done");
        println!("4. View Tasks");
        println!("Type 'exit' to return to the main menu");
        println!("--------------------");

        let cmd: String = input("Enter a command: ");

        match cmd.as_str() {
            "add task" | "1" => add_task(&mut tasks),
            "remove task" | "2" => remove_task(&mut tasks),
            "mark task done" | "3" => mark_task_done(&mut tasks),
            "view tasks" | "4" => view_task(&tasks),
            "exit" => {
                println!("Returning to the main menu...");
                return;
            }
            _ => println!("Invalid command."),
        }
    }
}
