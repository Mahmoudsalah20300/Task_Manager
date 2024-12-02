
mod tasks;


use std::io::{self,Write};
use tasks::TaskManager;





fn main()
{
    let mut task_manager = TaskManager::new();


    loop
    {
        println!("\nTo-Do List CLI");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Exit");
        println!("Choose an option: ");

        io::stdout().flush().unwrap(); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice{

            "1" => {
                println!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                task_manager.add_task(description.trim().to_string());
            }

            "2" => task_manager.list_tasks(),
            "3" => {
                println!("Enter task ID to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                if let Ok(id) = id.trim().parse()
                {
                    task_manager.mark_complete(id);
                }
                else
                {
                    println!("Invalid ID.");
                }
            }
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
    println!("Program Terminates");

}