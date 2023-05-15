use uuid::{uuid, Uuid};

#[derive(Debug)]
pub struct Portfolio {
    id: Uuid,
    pub name: String,
    pub user_id: Uuid,
}

impl Portfolio {
    pub fn new(name: String, user_id: Uuid) -> Self {
        Portfolio{
            id: Uuid::new_v4(),
            name,
            user_id
        }
    }
}