#[derive(Debug)]
pub struct Ticket {
    title: String,
    description: String,
    is_complete: bool,
    subtasks: Vec<Ticket>,
}

impl Ticket {
    pub fn new(title: String, description: String) -> Ticket {
        Ticket {
            title,
            description,
            is_complete: false,
            subtasks: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Swimlane {
    name: String,
    tickets: Vec<Ticket>,
}

impl Swimlane {
    pub fn new(name: String) -> Swimlane {
        Swimlane {
            name,
            tickets: vec!(),
        }
    }
}

