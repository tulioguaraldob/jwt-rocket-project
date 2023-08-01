use super::repository::TaskRepository;

pub struct TaskService {
    pub repository: TaskRepository,
}

// impl<repository: TaskRepository> TaskService<repository> {
//     fn get_all_tasks(&mut self, id: i32) -> Result<Vec<Task>, diesel::result::Error> {
//         let tasks = self.repository.get_all_tasks()?;
//         return Ok((tasks));
//     }

//     fn get_task_by_id(&mut self, id: i32) -> Result<Task, diesel::result::Error> {
//         let task = self.repository.get_task_by_id(id)?;
//         return Ok((task));
//     }

//     fn create_task(
//         &mut self,
//         task: crate::db::entity::task::TaskRequest,
//     ) -> Result<(), diesel::result::Error> {
//         self.repository.create_task(&task);
//         return Ok(());
//     }

//     fn delete_task(&mut self, id: i32) -> Result<(), diesel::result::Error> {
//         self.repository.delete_task(id);
//         return Ok(());
//     }
// }
