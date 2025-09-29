use super::RegexParser;

// =============================================================================
// STAR (*) TESTS
// =============================================================================

#[test]
fn star_zero_matches() {
    let parser = RegexParser::from("a*");
    assert_eq!(parser.parse("bcdef"), Some(0)); // matches empty string at start
}

#[test]
fn star_one_match() {
    let parser = RegexParser::from("a*");
    assert_eq!(parser.parse("abcdef"), Some(0));
}

#[test]
fn star_multiple_matches() {
    let parser = RegexParser::from("a*");
    assert_eq!(parser.parse("aaaabcdef"), Some(0));
}

#[test]
fn star_with_following_char() {
    let parser = RegexParser::from("a*b");
    assert_eq!(parser.parse("b"), Some(0)); // zero a's + b
    assert_eq!(parser.parse("ab"), Some(0)); // one a + b
    assert_eq!(parser.parse("aaab"), Some(0)); // multiple a's + b
    assert_eq!(parser.parse("aaaac"), None); // no b after a's
}

#[test]
fn star_in_middle() {
    let parser = RegexParser::from("xa*y");
    assert_eq!(parser.parse("xy"), Some(0)); // x + zero a's + y
    assert_eq!(parser.parse("xay"), Some(0)); // x + one a + y
    assert_eq!(parser.parse("xaaay"), Some(0)); // x + multiple a's + y
}
