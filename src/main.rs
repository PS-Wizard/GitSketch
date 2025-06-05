use std::process::Command;

use serde::Deserialize;
use tiny_http::{Response, Server};

#[derive(Deserialize, Debug)]

struct Payload {
    dates: Vec<String>,
}

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("GitSketch: Up And Running at :8080");

    for mut request in server.incoming_requests() {
        if request.method().as_str() == "POST" && request.url() == "/submit" {
            let mut content = String::new();
            request.as_reader().read_to_string(&mut content).unwrap();

            match serde_json::from_str::<Payload>(&content) {
                Ok(data) => {
                    println!("Got {:#?}", data.dates);
                    for date in data.dates {
                        let date_str = format!("{date}T12:00:00");
                        let cmd = format!(
                            "GIT_AUTHOR_DATE=\"{0}\" GIT_COMMITTER_DATE=\"{0}\" git commit --allow-empty -m \"PS-Wizard\"",
                            date_str
                        );
                        let _ = Command::new("sh")
                            .arg("-c")
                            .arg(cmd)
                            .status()
                            .expect("failed to execute commit");
                        println!("Made Commit for: {date}");
                    }

                    let response = Response::from_string("Success: You May Push To Github Now!")
                        .with_status_code(200);
                    let _ = request.respond(response);
                }
                Err(e) => {
                    let response =
                        Response::from_string(format!("Invalid Json: {}", e)).with_status_code(401);
                    let _ = request.respond(response);
                }
            }
        } else {
            let response =
                Response::from_string("Only POST to /submit Works: {}").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
