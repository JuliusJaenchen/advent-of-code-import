use std::sync::Arc;

use dirs::home_dir;
use tokio::fs;

use reqwest::{Client, StatusCode};
use reqwest::{cookie::Jar, Url};


#[tokio::main]
async fn main() { 
    let start_time = std::time::Instant::now();
    let mut args = std::env::args();
    args.next();

    let mut session = args.next().expect("missing first argument (session)");
    let year: u16 = args.next().unwrap_or(String::from("2021")).parse::<u16>().expect("year parsing error");

    let cookie_jar: Jar = Jar::default();
    session.insert_str(0, "session=");
    let domain: Url = "https://adventofcode.com".parse::<Url>().unwrap();
    cookie_jar.add_cookie_str(&session, &domain);


    let client: Client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider( Arc::new(cookie_jar))
        .build().unwrap();

    let client = Arc::new(client);

    println!("Downloading Puzzle inputs for {year}...");
    let mut join_handlers = Vec::new();
    for day in 1..26 {
        let cloned_client = client.clone();
        join_handlers.push(tokio::task::spawn(get_problem_data(cloned_client, year, day)));
    }
    
    for join_handler in join_handlers {
        join_handler.await.expect("task failed to execute");
    }
    println!("Took {:.2}s to download puzzle inputs. Good Luck!", start_time.elapsed().as_millis() as f64 / 1000.0);
}

async fn get_problem_data(client: Arc<Client>, year: u16, day: usize) {
    match fetch_problem_data(client, year, day).await {
        Some(problem_data) =>  persist_problem_data(year, day, problem_data).await,
        None => println!("No Input found for day {day}")
    }
}

async fn fetch_problem_data(client: Arc<Client>, year: u16, day: usize) -> Option<String> {
    let url: String = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = client.get(&url).send().await.ok().unwrap();

    match response.status() {
        StatusCode::OK => return response.text().await.ok(),
        StatusCode::BAD_REQUEST => {
            let reason = response.text().await.unwrap();
            match reason.as_str() {
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" => panic!("invalid session token"),
                _ => panic!("{reason}"),
            }
        },
        StatusCode::NOT_FOUND => return None,
        _ => {
            panic!("{}", response.status().to_string());
        }
    }
}

async fn persist_problem_data(year: u16, day: usize, input: String) {
    let raw_input = input.as_bytes();

    let mut target_path = home_dir().expect("couldn't find home directory");
    target_path.push("AdventOfCode_test");
    target_path.push(year.to_string());
    target_path.push(format!("day{:02}", day));
    fs::create_dir_all(&target_path).await.expect("unable to crate directory");
    target_path.push("input.txt");
    fs::write(&target_path, raw_input).await.expect("failed to save input data");
}
