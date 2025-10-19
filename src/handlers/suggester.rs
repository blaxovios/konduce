/// Simple rust filter map function to suggest commands based on user input.
pub fn suggester(val: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let command_suggestions = vec![
        "get",
        "set",
        "delete",
        "list",
        "search",
        "help",
        "exit"
    ];

    let lowered_suggestion = val.to_ascii_lowercase();

    let out = command_suggestions
        .iter()
        .copied() // &str (not &&str)
        .filter(|s| s.to_ascii_lowercase().contains(&lowered_suggestion))
        .map(|s| s.to_string())
        .collect();

    Ok(out)
}