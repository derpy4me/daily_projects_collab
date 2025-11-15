use serde_json::{json, Value};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let raw_event_data = json!({
        "event_type": "user.created",
        "timestamp": "2023-10-27T10:00:00Z",
        "data": {
            "id": 4591,
            "username": "rusty",
            "email": "user@example.com",
            "is_admin": false,
            "preferences": {
                "notifications": true,
                "theme": "dark"
            },
        },
        "internal_flags": ["flag_a", "flag_b"]
    });

    println!("{}", serde_json::to_string_pretty(&raw_event_data)?);
    println!("------------------------------------------------\n");

    let user_a_filter = "data.{external_id: id, handle: username}";
    let user_b_filter = "{event: event_type, user_prefs: data.preferences}";
    let user_c_filter = "data.is_admin";

    let _ = process_webhook(&raw_event_data, user_a_filter, "User A");
    let _ = process_webhook(&raw_event_data, user_b_filter, "User B");
    let _ = process_webhook(&raw_event_data, user_c_filter, "User C");

    Ok(())
}

fn process_webhook(payload: &Value, expression: &str, user_label: &str) -> Result<(), Box<dyn Error>> {
    let expr = jmespath::compile(expression)?;
    let result = expr.search(payload)?;
    println!("Destination: {}", user_label);
    println!("Filter:      {}", expression);

    if result.is_null() {
        println!("Result:     [Payload Filtered Out / Empty");
    } else {
        let output: Value = serde_json::from_str(&result.to_string())?;
        println!("Sending:     {}", output);
    }

    println!("\n");

    Ok(())
}
