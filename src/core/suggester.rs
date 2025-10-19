/// Simple rust filter map function to suggest commands based on user input.
use crate::core::did_you_mean::did_you_mean;


pub fn suggester(val: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let command_suggestions: Vec<&'static str> = vec![
        "get",
        "set",
        "delete",
        "list",
        "search",
        "help",
        "exit"
    ];

    let lowered_suggestion = val.to_ascii_lowercase();

    let out:Vec<String> = command_suggestions
        .iter()
        .copied() // &str (not &&str)
        .filter(|s| s.to_ascii_lowercase().contains(&lowered_suggestion))
        .map(|s| s.to_string())
        .collect();

    // If no substring hits, fall back to did_you_mean
    if out.is_empty()
    {
        if let Some((best, _dist)) = did_you_mean(&lowered_suggestion, command_suggestions) {
            return Ok(vec![best.to_string()]);
        }
    }

    Ok(out)
}