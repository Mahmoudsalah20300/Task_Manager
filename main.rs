/******************************
 * By: Mahmoud Ali
 * Date: 6.12.2024 - 12.12.2024
 * 
 * ***************************/




// Define to the current file the file tasks 
mod tasks;

// Import the ability to write and read 
use std::io::{self,Write};
// form module/file/lib tasks import TaskManager  
use tasks::TaskManager;




// The main function 
fn main()
{
    // Create a new Object of TaskManager
    let mut task_manager = TaskManager::new();

    // Infinite loop to handle task options
    loop
    {
        // Print the Options 
        println!("\nTo-Do List CLI");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Complete");
        println!("4. Exit");
        println!("Choose an option: ");

        // send the output immediatley to the stdout 
        // and store the result (sucess or faileur) using .unwrap()
        io::stdout().flush().unwrap(); 
        
        // Create a new string instance
        let mut input = String::new();
        // read the input of the user 
        io::stdin().read_line(&mut input).unwrap();
        // take the choice but remove the newline "\n" using .trim();
        let choice = input.trim();

        // decide action based on input
        match choice{
            // In case of adding task 
            "1" => {        
                println!("Enter task description: ");
                // Send it directley without any delay 
                io::stdout().flush().unwrap();
                // Create string instance 
                let mut description = String::new();
                // Read the user's input
                io::stdin().read_line(&mut description).unwrap();
                // add the new task with ID given
                task_manager.add_task(description.trim().to_string());
            }
            // If choice is 2 then display the tasks 
            "2" => task_manager.list_tasks(),
            "3" => {

                println!("Enter task ID to mark as complete: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                // If the choice is 3 then mark the given input(if right) as completed
                if let Ok(id) = id.trim().parse()
                {
                    task_manager.mark_complete(id);
                }
                else
                {
                    println!("Invalid ID.");
                }
            }
            // In case of choice 4 then end the program 
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
    println!("Program Terminates");

}