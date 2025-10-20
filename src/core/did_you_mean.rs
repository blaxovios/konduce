use rapidfuzz::distance::levenshtein;

use crate::constants::LEVENSHTEIN_MAX_DISTANCE;


pub fn did_you_mean<'a>(input: &str, options: impl IntoIterator<Item = &'a str>) -> Option<(&'a str, usize)> {
    let needle = input.to_ascii_lowercase();

    // Find the closest candidate; only return it if it's within the threshold.
    let (best, dist) = options
        .into_iter()
        .map(|opt| {
            let d = levenshtein::distance(needle.chars(), opt.to_ascii_lowercase().chars());
            (opt, d)
        })
        .min_by_key(|(_, d)| *d)?; // early-exit if there were no options

    (dist <= LEVENSHTEIN_MAX_DISTANCE).then_some((best, dist))
}
