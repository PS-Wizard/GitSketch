use std::process::Command;

use serde::Deserialize;
use tiny_http::{Header, Method, Response, Server};

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
                    }

                    let response = Response::from_string("OK")
                        .with_header(
                            Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap(),
                        )
                        .with_header(
                            Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(
                                "Access-Control-Allow-Methods",
                                "POST, GET, OPTIONS",
                            )
                            .unwrap(),
                        )
                        .with_status_code(200);

                    let _ = request.respond(response);
                }
                Err(e) => {
                    let response =
                        Response::from_string(format!("Invalid Json: {}", e)).with_status_code(401);
                    let _ = request.respond(response);
                }
            }
        } else if request.method() == &Method::Options {
            let response = Response::empty(200)
                .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap())
                .with_header(
                    Header::from_bytes("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
                        .unwrap(),
                )
                .with_header(Header::from_bytes("Access-Control-Allow-Headers", "*").unwrap());
            let _ = request.respond(response);
            continue;
        } else {
            let response =
                Response::from_string("Only POST to /submit Works: {}").with_status_code(404);
            let _ = request.respond(response);
        }
    }
}
