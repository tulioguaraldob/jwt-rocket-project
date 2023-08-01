use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::db::entity::task::{Task, TaskRequest};

// pub trait ITaskRepository {
//     fn get_all_tasks(&mut self) -> Result<Vec<Task>, diesel::result::Error>;
//     fn get_task_by_id(&mut self, id: i32) -> Result<Task, diesel::result::Error>;
//     fn create_task(&mut self, task: &TaskRequest) -> Result<(), diesel::result::Error>;
//     fn delete_task(&mut self, id: i32) -> Result<(), diesel::result::Error>;
// }

pub struct TaskRepository {
    pub db: PgConnection,
}

impl TaskRepository {
    pub fn get_all_tasks(&mut self) -> Result<Vec<Task>, diesel::result::Error> {
        use crate::schema::tasks::dsl::*;

        let all_tasks = tasks.load::<Task>(&mut self.db)?;
        return Ok(all_tasks);
    }

    pub fn get_task_by_id(&mut self, task_id: i32) -> Result<Task, diesel::result::Error> {
        use crate::schema::tasks::dsl::*;

        let task = tasks.find(task_id).first::<Task>(&mut self.db)?;
        return Ok(task);
    }

    pub fn create_task(&mut self, task: &TaskRequest) -> Result<(), diesel::result::Error> {
        use crate::schema::tasks;

        diesel::insert_into(tasks::table)
            .values(task)
            .execute(&mut self.db)?;
        return Ok(());
    }

    pub fn delete_task(&mut self, task_id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::tasks::dsl::*;

        diesel::delete(tasks.find(task_id)).execute(&mut self.db)?;
        return Ok(());
    }
}
