mod errors;

use std::thread::sleep;
use std::time::Duration;
use lambda_http::{RequestExt, Response, run, service_fn};
use lambda_http::Error;
use tracing::{error, info};
use crate::errors::ParseDurationError;

fn parse_duration(value: &str) -> Result<u64, ParseDurationError> {
    let n_value =
        value
            .parse::<u64>()
            .map_err(|e| ParseDurationError::InvalidDuration {
                input: value.into(),
                parse: e,
            })?;

    if n_value > 10000 {
        Err(ParseDurationError::DurationTooLong { input: n_value, max: 10000 })
    } else {
        Ok(n_value)
    }
}

async fn process_request(request: lambda_http::Request) -> Result<u64, ParseDurationError> {
    let query = request.query_string_parameters();
    let wait_str = query.first("wait").ok_or(ParseDurationError::DurationMissing)?;
    let wait = parse_duration(wait_str)?;

    info!("waiting for {wait} milliseconds");
    sleep(Duration::from_millis(wait));
    Ok(wait)
}

async fn function_handler_http(request: lambda_http::Request) -> Result<Response<String>, Error> {
    let response = process_request(request).await
        .map(|wait|
            Response::new(
                format!("waited for {wait} milliseconds")
            ))
        .unwrap_or_else(|e| {
            error!("error while processing: {e}");
            Response::new(
                format!("invalid input")
            )
        }
        );
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler_http)).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use assert_matches::assert_matches;
    use lambda_http::aws_lambda_events::query_map::QueryMap;
    use lambda_http::{Body};
    use lambda_http::http::Request;

    use super::*;

    fn request_with_wait_parameter(key: &str, value: &str) -> Request<Body> {
        return RequestExt::with_query_string_parameters(
            Request::new("".into()),
            QueryMap::from(HashMap::<String, String>::from([(String::from(key), String::from(value))])),
        );
    }

    #[tokio::test]
    async fn test_valid_wait() {
        let response = function_handler_http(
            request_with_wait_parameter("wait", "500")
        ).await.unwrap();

        assert_eq!(response.body(), "waited for 500 milliseconds");
    }

    #[tokio::test]
    async fn test_missing_wait() {
        let response = process_request(
            request_with_wait_parameter("waitx", "500")
        ).await;

        assert_matches!(response, Err(ParseDurationError::DurationMissing));
    }

    #[tokio::test]
    async fn test_invalid_wait() {
        let response = process_request(
            request_with_wait_parameter("wait", "500x")
        ).await;

        assert_matches!(response, Err(ParseDurationError::InvalidDuration { input: _ , parse: _ }));
    }

    #[tokio::test]
    async fn test_wait_too_long() {
        let response = process_request(
            request_with_wait_parameter("wait", "5000000")
        ).await;

        assert_matches!(response, Err(ParseDurationError::DurationTooLong { input: 5000000, max: 10000 }));
    }
}
