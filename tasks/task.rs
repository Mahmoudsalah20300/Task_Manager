

// This traits will be used later
// To print a human readable fromat
// of the struct
#[derive(Debug, Clone)]

/********************************************
The following struct is a public struct 
that contains  informations about tasks   
like id, description and if it is completed
or not.   
pub: means can be accessed form outside of struct. 
**********************************************/
pub struct Task{

    pub id:usize,
    pub description: String,
    pub completed: bool,

}


/***************************************
 here impl is keyword to define that the    
 upcoming class contains implementation for  
 somthing related to the Task struct.
 ***************************************/
impl Task{
    // create a function called new and initailize 
    // it with
    // id and description and comleted by defult as 
    // false
    pub fn new(id: usize, description: String) -> Self{
      Self{  
            id,
            description,
            completed: false,
      }
    }
    // Define a function to mark the task as completed 
    pub fn mark_complete(&mut self)
    {
        self.completed = true;
    }

}