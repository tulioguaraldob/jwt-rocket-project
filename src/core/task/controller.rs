use rocket::{get, response::content::Json};

use crate::db::entity::task::Task;

use super::service::TaskService;

pub struct TaskController {
    pub service: TaskService,
}

impl TaskController {
    // #[get("/tasks/<id>", format = "application/json")]
    pub fn read(&mut self, id: i32) -> Option<Json<Task>> {
        let task = self.service.repository.get_task_by_id(id).unwrap();
        Some(Json(task))
    }
}
