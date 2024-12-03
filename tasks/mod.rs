
/******************************
 * By: Mahmoud Ali
 * Date: 6.12.2024 - 12.12.2024
 * 
 * ***************************/


// include the task module 
mod task;

// fron the task module use Task struct 
use self::task::Task;


// Create a struct which is Task manager 
pub struct TaskManager{
    
    // Create a vector to store diffetrent tasks 
    tasks: Vec<Task>,
    // update the id of the will-be task
    next_id: usize,
}


// implement Tasks related to the TaskManager
impl TaskManager{

    // Creat a new function to initialize the Task (Like Constructors)
    pub fn new() -> Self{
        Self{
            tasks: Vec::new(), 
            next_id: 1,
        }
    }

    // add a new task with description 
    pub fn add_task(&mut self, description: String)
    {
        // Create new task 
        let task = Task:: new(self.next_id,description);
        // store it in the running tasks 
        self.tasks.push(task);
        // Updeate the id
        self.next_id += 1;
    }

    // Make a list of tasks 
    pub fn list_tasks(&self)
    {
        // loop over the task to know which task is done and which is pending
        for task in &self.tasks{
            println!("{}. {} [{}]",task.id,
            task.description,
            if task.completed {"Done"} else {"Pending"});
        }
    }


    // mark the system as complete 
    pub fn mark_complete(&mut self, id: usize)
    {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id)
        {
            task.mark_complete();
        }
        else 
        {
            println!("Task with ID {} not found.",id);
        }
    }
}





