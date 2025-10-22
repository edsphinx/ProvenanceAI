// HTTP Outcalls Utility Module
// Provides reusable functions for making HTTPS requests from the canister

use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse,
    TransformArgs, TransformContext, TransformFunc,
};

// ==============================================================================
// Transform Function
// ==============================================================================

/// Transform function to sanitize HTTP responses
/// This is required by ICP to ensure consensus on HTTP outcall responses
#[ic_cdk::query]
fn http_transform(args: TransformArgs) -> HttpResponse {
    let mut res = args.response;

    // Remove headers to reduce size and ensure consensus
    res.headers = vec![];

    // Limit response body size to 1MB
    if res.body.len() > 1024 * 1024 {
        ic_cdk::trap("HTTP response body too large (>1MB)");
    }

    res
}

// ==============================================================================
// HTTP Request Helper
// ==============================================================================

/// Make an HTTP request with automatic cycle management
///
/// # Arguments
/// * `url` - The URL to request
/// * `method` - HTTP method (GET, POST, etc.)
/// * `headers` - HTTP headers
/// * `body` - Optional request body
///
/// # Returns
/// * `Result<Vec<u8>, String>` - Response body or error message
pub async fn make_http_request(
    url: String,
    method: HttpMethod,
    headers: Vec<HttpHeader>,
    body: Option<Vec<u8>>,
) -> Result<Vec<u8>, String> {
    ic_cdk::println!("📡 HTTP Outcall: {} {}", method_to_string(&method), url);

    // Create transform context
    let transform_context = TransformContext {
        function: TransformFunc(candid::Func {
            principal: ic_cdk::api::id(),
            method: "http_transform".to_string(),
        }),
        context: vec![],
    };

    // Build request
    let request = CanisterHttpRequestArgument {
        url: url.clone(),
        method,
        body: body.clone(),
        max_response_bytes: Some(1024 * 1024), // 1MB max
        headers: headers.clone(),
        transform: Some(transform_context),
    };

    // Calculate cycles cost
    // Formula based on IC documentation:
    // base: (3_000_000 + 60_000 * nodes) * nodes
    // per-byte request: 400 * request_bytes * nodes
    // per-byte response: 800 * max_response_bytes * nodes
    // Where nodes = 13 for subnet consensus
    let nodes = 13u128;
    let request_size = body.as_ref().map_or(0, |b| b.len()) as u128;
    let max_response_bytes = 1024 * 1024u128; // 1MB

    let base_cost = (3_000_000 + (60_000 * nodes)) * nodes;
    let request_cost = 400 * request_size * nodes;
    let response_cost = 800 * max_response_bytes * nodes;
    let total_cycles = base_cost + request_cost + response_cost;

    // Add 20% buffer for safety
    let total_cycles = (total_cycles * 12) / 10;

    ic_cdk::println!("   💰 Cycles: {}", total_cycles);

    // Make the request
    match http_request(request, total_cycles).await {
        Ok((response,)) => {
            let status_code: u32 = response.status.0.try_into().unwrap_or(500);
            if status_code >= 200 && status_code < 300 {
                ic_cdk::println!("   ✅ Response: {} bytes (status {})", response.body.len(), status_code);
                Ok(response.body)
            } else {
                let error_msg = format!(
                    "HTTP Error {}: {}",
                    status_code,
                    String::from_utf8_lossy(&response.body)
                );
                ic_cdk::println!("   ❌ {}", error_msg);
                Err(error_msg)
            }
        }
        Err((code, msg)) => {
            let error_msg = format!("HTTP Outcall Failed: {:?} - {}", code, msg);
            ic_cdk::println!("   ❌ {}", error_msg);
            Err(error_msg)
        }
    }
}

// ==============================================================================
// Helper Functions
// ==============================================================================

fn method_to_string(method: &HttpMethod) -> &'static str {
    match method {
        HttpMethod::GET => "GET",
        HttpMethod::POST => "POST",
        HttpMethod::HEAD => "HEAD",
    }
}

/// Create a standard JSON header
pub fn json_header() -> HttpHeader {
    HttpHeader {
        name: "Content-Type".to_string(),
        value: "application/json".to_string(),
    }
}

/// Create an authorization header
pub fn auth_header(token: &str) -> HttpHeader {
    HttpHeader {
        name: "Authorization".to_string(),
        value: format!("Bearer {}", token),
    }
}

// ==============================================================================
// Convenience Functions
// ==============================================================================

/// Response structure for HTTP requests
#[derive(Debug, Clone)]
pub struct HttpOutcallResponse {
    pub status: u32,
    pub body: String,
}

/// Make an HTTP POST request with JSON payload
///
/// # Arguments
/// * `url` - The URL to POST to
/// * `json_body` - JSON string to send as body
/// * `cycles` - Cycles to allocate for the request
///
/// # Returns
/// * `Result<HttpOutcallResponse, String>` - Response or error
pub async fn http_post(
    url: &str,
    json_body: &str,
    cycles: u128,
) -> Result<HttpOutcallResponse, String> {
    ic_cdk::println!("📡 HTTP POST: {}", url);
    ic_cdk::println!("   Body length: {} bytes", json_body.len());

    // Create transform context
    let transform_context = TransformContext {
        function: TransformFunc(candid::Func {
            principal: ic_cdk::api::id(),
            method: "http_transform".to_string(),
        }),
        context: vec![],
    };

    // Build request
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::POST,
        body: Some(json_body.as_bytes().to_vec()),
        max_response_bytes: Some(10_000), // 10KB should be enough for tx hash response
        headers: vec![json_header()],
        transform: Some(transform_context),
    };

    ic_cdk::println!("   💰 Cycles allocated: {}", cycles);

    // Make the request
    match http_request(request, cycles).await {
        Ok((response,)) => {
            let status_code: u32 = response.status.0.try_into().unwrap_or(500);
            let body_str = String::from_utf8_lossy(&response.body).to_string();

            ic_cdk::println!("   ✅ Response status: {}", status_code);
            ic_cdk::println!("   Response body: {}", body_str);

            Ok(HttpOutcallResponse {
                status: status_code,
                body: body_str,
            })
        }
        Err((code, msg)) => {
            let error_msg = format!("HTTP Outcall Failed: {:?} - {}", code, msg);
            ic_cdk::println!("   ❌ {}", error_msg);
            Err(error_msg)
        }
    }
}
