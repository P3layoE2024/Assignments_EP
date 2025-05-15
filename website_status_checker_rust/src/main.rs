// CSCI FINAL PROJECT
use std::{
    env,                                    // Environment files
    fs::File,                               // Files
    io::{self, BufRead, Write},             // Input/Output
    path::Path,                             // Read from file
    sync::{mpsc, Arc, Mutex},               // Concurrency 
    thread,                                 // Threads
    time::{Duration, Instant, SystemTime},  // std Time library
};
use reqwest::blocking::Client;

struct WebsiteStatus {
    url: String,
    action_status: Result<u16, String>,
    response_time: Duration,
    timestamp: SystemTime,
}
// Breaks terminal input into necessary variables in main
fn parse_args() -> (Vec<String>, usize, u64, usize) {
    let args: Vec<String> = env::args().collect();
    let mut urls = Vec::new();
    let mut file_path = None;
    let mut workers = 1;
    let mut timeout = 5;
    let mut retries = 0;
    // Process command line input
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                if i + 1 >= args.len() {
                    eprintln!("--file requires a path");
                    std::process::exit(2);
                }
                file_path = Some(args[i + 1].clone());
                i += 1;
            }
            "--workers" => {
                workers = args[i + 1].parse().unwrap_or(workers);
                i += 1;
            }
            "--timeout" => {
                timeout = args[i + 1].parse().unwrap_or(timeout);
                i += 1;
            }
            "--retries" => {
                retries = args[i + 1].parse().unwrap_or(retries);
                i += 1;
            }
            _ => {
                if args[i].starts_with("--") {
                    eprintln!("Unknown option: {}", args[i]);
                    std::process::exit(2);
                } else {
                    urls.push(args[i].clone());
                }
            }
        }
        i += 1;
    }
    // Read file if given
    if let Some(path) = file_path {
        if let Ok(lines) = read_lines(path) {
            for line in lines.flatten() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    urls.push(trimmed.to_string());
                }
            }
        }
    }
    // If no urls are present, give example input for terminal and exit status 2
    if urls.is_empty() {
        eprintln!("Usage: website_checker [--file sites.txt] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    (urls, workers, timeout, retries)
}
// Read file lines
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// Function gets the response and response time from the url given 
fn check_website(client: &Client, url: &String, retries: usize) -> WebsiteStatus {
    let mut attempts = 0;
    let start = Instant::now();
    let result;

    loop {
        let now = SystemTime::now();
        let response = client.get(url).send();
        

        match response {
            Ok(resp) => {
                let code = resp.status().as_u16();
                result = WebsiteStatus {
                    url: url.to_string(),
                    action_status: Ok(code),
                    response_time: start.elapsed(),
                    timestamp: now,
                };
                break;
            }
            Err(err) => {
                if attempts < retries {
                    attempts += 1;
                    thread::sleep(Duration::from_millis(10));
                    continue;
                } else {
                    result = WebsiteStatus {
                        url: url.to_string(),
                        action_status: Err(err.to_string()),
                        response_time: start.elapsed(),
                        timestamp: now,
                    };
                    break;
                }
            }
        }
    }
    result
}

fn main() {
    let (urls, workers, timeout, retries) = parse_args();
    let (job_tx, job_rx) = mpsc::channel();
    let (res_tx, res_rx) = mpsc::channel();

    let job_rx = Arc::new(Mutex::new(job_rx));

    let client = Arc::new(
        Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap(),
    );

    for url in &urls {
        job_tx.send(url.clone()).unwrap();
    }

    for _ in 0..workers {
        let job_clone = Arc::clone(&job_rx);
        let res_clone = res_tx.clone();
        let client = client.clone();

        thread::spawn(move || {
            
            let url = job_clone.lock().unwrap().recv().unwrap();
            let result = check_website(&client, &url, retries);
            res_clone.send(result).unwrap();
            
        });
    }

    // Recieve all results from the result into a vector
    // Display condition if Ok or Err
    let mut results = Vec::new();
    for _ in 0..urls.len() {
        if let Ok(status) = res_rx.recv() {
            match &status.action_status {
                Ok(code) => println!("OK   {}  {}", status.url, code),
                Err(err) => println!("ERR  {}  {}", status.url, err),
            }
            results.push(status);
        }
    }
    // Drop input channel
    drop(job_tx);

    // Print all results onto JSON file
    let mut file = File::create("status.json").unwrap();
    write!(file, "[").unwrap();
    for (i, status) in results.iter().enumerate() {
        let obj = format!(
            "{{\"url\":\"{}\",\"status\":{},\"response_time_ms\":{},\"timestamp\":\"{:?}\"}}",
            status.url,
            match &status.action_status {
                Ok(code) => code.to_string(),
                Err(err) => format!("\"{}\"", err.replace('"', "'")),
            },
            status.response_time.as_millis(),
            status.timestamp
        );
        write!(file, "{}{}", obj, if i + 1 < results.len() { "," } else { "" }).unwrap();
    }
    write!(file, "]").unwrap();
}
