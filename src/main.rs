pub mod core;
pub mod db;
pub mod schema;

fn main() -> Result<(), diesel::result::Error> {
    dotenvy::dotenv().ok();

    println!("- - - - -");
    println!("- - - - -");
    println!("- - - - -");

    println!("{}", std::env::var("DATABASE_URL").unwrap());

    println!("- - - - -");
    println!("- - - - -");
    println!("- - - - -");

    let db = db::database::open_connection();
    let repository = core::task::repository::TaskRepository { db: db };
    let service = &mut core::task::service::TaskService {
        repository: repository,
    };

    let tasks = service.repository.get_all_tasks()?;

    println!("{:?}", tasks);

    println!("- - - - -");
    println!("- - - - -");
    println!("- - - - -");

    println!("{}, {}", tasks[49].id, tasks[49].title);

    println!("- - - - -");
    println!("- - - - -");
    println!("- - - - -");

    Ok(())
}
