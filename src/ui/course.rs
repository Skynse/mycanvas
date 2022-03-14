use serde_json::{Result, Value};

use reqwest::{Client, Url};

fn get_api_key() -> String {
    let config_file = std::fs::read_to_string("config.json").expect("Failed to read config.json");
    let config: Value = serde_json::from_str(&config_file).expect("Failed to parse config.json");
    config["key"].as_str().expect("Failed to parse api_key").to_string()
}

#[derive(Debug, Clone)]
pub struct Course {
    name: String,
    id: String,
}

impl Course {
    
    fn new(name: String, id:String) -> Course {
        Course { name, id }
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }
}

#[tokio::main]
pub async fn get_courses() -> Result<Vec<Course>, > {
    let api_key = get_api_key();
    let client = Client::builder().build().unwrap();
    let res = client.get(Url::parse(
        "https://canvas.instructure.com/api/v1/courses").unwrap())
        .header("Authorization",format!("Bearer {}", api_key)).send().await.unwrap();
    let res_json: Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
    let courses: Vec<Course> = res_json.as_array().unwrap().iter().map(|course| {
        Course::new(
    course["course_code"].as_str().unwrap().to_string(), 
    course["course_id"].as_str().unwrap().to_string())
    }).collect();
    Ok(courses)
}

#[tokio::main]
pub async fn get_assignments(course_id: String) -> Result<Vec<String>, > {
    let api_key = get_api_key();
    let client = Client::builder().build().unwrap();
    let url = format!("https://canvas.instructure.com/api/v1/courses/{}/assignments", course_id);
    let res = client.get(Url::parse(&url).unwrap())
        .header("Authorization",format!("Bearer {}", api_key)).send().await.unwrap();
    let res_json: Value = serde_json::from_str(&res.text().await.unwrap()).unwrap();
    let assignments: Vec<String> = res_json.as_array().unwrap().iter().map(|assignment| {
        assignment["name"].as_str().unwrap().to_owned()
    }).collect();
    Ok(assignments)
}


#[test]
fn test_get_courses() {
    let courses = get_courses();
    assert_eq!(courses.is_ok(), true);
}

