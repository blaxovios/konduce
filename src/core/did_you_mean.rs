use rapidfuzz::distance::levenshtein;

use crate::constants::LEVENSHTEIN_MAX_DISTANCE;


pub fn did_you_mean<'l>(input: &str, options: Vec<&'l str>) -> Option<(&'l str, usize)> {
    let input_lowercase = input.to_ascii_lowercase();
    
    if options.is_empty() {
        return None;
    }

    options
        .into_iter()
        .map(|option| {let distance = levenshtein::distance(input_lowercase.chars(), option.to_ascii_lowercase().chars());
            (option, distance)
        })
        .filter(|(_, distance)| *distance <= LEVENSHTEIN_MAX_DISTANCE)
        .min_by_key(|(_, distance)| *distance)
        .and_then(|(best, dist)| (dist <= LEVENSHTEIN_MAX_DISTANCE).then_some((best, dist)))
}
