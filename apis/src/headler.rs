use rocket::serde::json::Json;

#[derive(Deserialize, Debug)]
pub struct Task2 {
    pub description: String,
    pub complete: bool,
}

#[post("/body-data/json", data = "<task>")]
pub fn body_data_json(task: Json<Task2>) -> String {
    println!("{}", task.0.description);
    println!("{}", task.0.complete);
    format!("{:?}", task.0)
}
