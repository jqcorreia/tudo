#[derive(Debug)]
pub struct FuzzyMatch {
    pub value: String,
    pub score: u32,
}

pub fn basic(filter: String, candidates: &[String]) -> Option<Vec<FuzzyMatch>> {
    let result: Vec<FuzzyMatch> = candidates
        .iter()
        .filter(|c| c.starts_with(&filter))
        .map(|c| FuzzyMatch {
            value: c.to_string(),
            score: 0,
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
