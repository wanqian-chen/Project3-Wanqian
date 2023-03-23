use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    command: String,
    place: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let command = event.payload.command;
    let place = event.payload.place;

    let res: String;
    if command == "date" {
        if place == "tokyo" {
            res = get_info::get_date_tokyo();
        } else if place == "paris" {
            res = get_info::get_date_paris();
        } else if place == "new york" {
            res = get_info::get_date_ny();
        } else if place == "london" {
            res = get_info::get_date_london();
        } else if place == "sydney" {
            res = get_info::get_date_sydney();
        } else if place == "san francisco" {
            res = get_info::get_date_sf();
        } else if place == "beijing" {
            res = get_info::get_date_beijing();
        } else {
            res = "Invalid place".to_string();
        }
    } else if command == "time" {
        if place == "tokyo" {
            res = get_info::get_time_tokyo();
        } else if place == "paris" {
            res = get_info::get_time_paris();
        } else if place == "new york" {
            res = get_info::get_time_ny();
        } else if place == "london" {
            res = get_info::get_time_london();
        } else if place == "sydney" {
            res = get_info::get_time_sydney();
        } else if place == "san francisco" {
            res = get_info::get_time_sf();
        } else if place == "beijing" {
            res = get_info::get_time_beijing();
        } else {
            res = "Invalid place".to_string();
        }
    } else {
        res = "Invalid command".to_string();
    }

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("The {} in {} is {}", command, place, res),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
