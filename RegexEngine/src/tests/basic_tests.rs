use super::RegexParser;

// =============================================================================
// BASIC SINGLE CHARACTER TESTS
// =============================================================================

#[test]
fn single_letter_match_first() {
    let parser = RegexParser::from("a");
    assert_eq!(parser.parse("abaaaa"), Some(0));
}

#[test]
fn single_letter_match_middle() {
    let parser = RegexParser::from("b");
    assert_eq!(parser.parse("abaaaa"), Some(1));
}

#[test]
fn single_letter_no_match() {
    let parser = RegexParser::from("z");
    assert_eq!(parser.parse("abaaaa"), None);
}
