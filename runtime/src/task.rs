pub struct Task {
    pub id: u32,
    pub name: String,
}

impl Task {
    pub fn new(id: u32, name: String) -> Self {
        Task { id, name }
    }
}
