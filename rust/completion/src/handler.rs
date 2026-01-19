pub fn autocomplete(query: &str, limit: usize, dictionary: &[String]) -> Vec<String> {
    if query.is_empty() {
        return Vec::new();
    }
    
    let query_lower = query.to_lowercase();
    
    let mut matches: Vec<String> = dictionary
        .iter()
        .filter(|term| term.to_lowercase().starts_with(&query_lower))
        .cloned()
        .collect();
    
    matches.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    
    matches.truncate(limit);
    matches
}

pub fn handle_request(request: &str, dictionary: &[String]) -> String {
    let lines: Vec<&str> = request.lines().collect();
    
    if lines.is_empty() {
        return error_response(400, "Bad Request");
    }
    
    let request_line = lines[0];
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return error_response(400, "Bad Request");
    }
    
    let path = parts[1];
    
    if !path.starts_with("/autocomplete") {
        return error_response(404, "Not Found");
    }
    
    let (query, limit) = parse_query_params(path);
    
    let results = autocomplete(&query, limit, dictionary);
    
    // Build JSON response
    let json_response = build_json(&results);
    
    // Build HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        json_response.len(),
        json_response
    );
    
    response
}

fn parse_query_params(path: &str) -> (String, usize) {
    let mut query = String::new();
    let mut limit = 5usize;
    
    if let Some(query_start) = path.find('?') {
        let query_string = &path[query_start + 1..];
        
        for param in query_string.split('&') {
            if let Some(eq_pos) = param.find('=') {
                let key = &param[..eq_pos];
                let value = &param[eq_pos + 1..];
                
                if key == "query" {
                    query = url_decode(value);
                } else if key == "limit" {
                    if let Ok(l) = value.parse::<usize>() {
                        limit = l;
                    }
                }
            }
        }
    }
    
    (query, limit)
}

fn url_decode(s: &str) -> String {
    s.replace("%20", " ")
        .replace("%3A", ":")
        .replace("%2F", "/")
        // Add more decodings as needed
}

fn build_json(results: &[String]) -> String {
    let json_items: Vec<String> = results
        .iter()
        .map(|s| format!("\"{}\"", escape_json(s)))
        .collect();
    
    format!("[{}]", json_items.join(", "))
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn error_response(code: u16, message: &str) -> String {
    let error_json = format!("{{\"error\": \"{}\"}}", message);
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        code,
        message,
        error_json.len(),
        error_json
    )
}