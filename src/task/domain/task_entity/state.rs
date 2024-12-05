#[derive(Debug, Clone, PartialEq)]

pub enum  StateLifeCycle{
    ToDo,
    InProgress,
    Done
}


pub enum StateManagement {
    Created, // when it's created but not modified
    Modifying, 
    Completed,
    Cancelled,
    Archived
}
