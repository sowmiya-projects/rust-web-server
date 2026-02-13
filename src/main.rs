use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("Server running on port 8080...");

    for request in server.incoming_requests() {
        let response = Response::from_string("Rust app is running on VM!");
        let _ = request.respond(response);
    }
}

