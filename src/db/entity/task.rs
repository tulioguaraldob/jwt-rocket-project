use diesel::{Queryable, Selectable, Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct TaskRequest {
    pub title: String,
    pub completed: bool,
}
