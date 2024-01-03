#[derive(Debug)]
pub struct FuzzyMatch {
    pub value: String,
    pub score: u32,
    pub original_idx: usize,
}

pub fn basic(filter: String, candidates: &[String]) -> Option<Vec<FuzzyMatch>> {
    let result: Vec<FuzzyMatch> = candidates
        .iter()
        .enumerate()
        .filter(|(_, c)| c.to_lowercase().starts_with(&filter.to_lowercase()))
        .map(|(i, c)| FuzzyMatch {
            value: c.to_string(),
            score: 0,
            original_idx: i,
        })
        .collect();

    match result.len() {
        0 => None,
        _ => Some(result),
    }
}

// fn fuzzy(filter: String, candidates: [String]) -> [FuzzyMatch] {}
#[cfg(test)]
mod tests {
    use crate::utils::fuzzy::basic;
    #[test]
    fn test_basic_search() {
        dbg!("hhhhhhhheeelo");
        dbg!(basic(
            String::from("hello"),
            &["hello".into(), "foo".into(), "hello world".into()]
        ));
    }
}
