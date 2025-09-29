use super::RegexParser;

// =============================================================================
// COMPLEX COMBINATION TESTS
// =============================================================================

#[test]
fn or_with_concatenation() {
    let parser = RegexParser::from("ab|cd");
    assert_eq!(parser.parse("abef"), Some(0));
    assert_eq!(parser.parse("cdef"), Some(0));
    assert_eq!(parser.parse("acbd"), None);
    assert_eq!(parser.parse("xyab"), Some(2));
}

#[test]
fn or_with_star() {
    let parser = RegexParser::from("a*|b*");
    assert_eq!(parser.parse("aaa"), Some(0));
    assert_eq!(parser.parse("bbb"), Some(0));
    assert_eq!(parser.parse("c"), Some(0)); // matches empty string
}

#[test]
fn star_with_or() {
    let parser = RegexParser::from("(a|b)*");
    assert_eq!(parser.parse("ababab"), Some(0));
    assert_eq!(parser.parse("aaaaaa"), Some(0));
    assert_eq!(parser.parse("bbbbbb"), Some(0));
    assert_eq!(parser.parse("c"), Some(0)); // matches empty string
}

#[test]
fn complex_pattern_1() {
    let parser = RegexParser::from("a*b|c");
    assert_eq!(parser.parse("b"), Some(0)); // zero a's + b
    assert_eq!(parser.parse("ab"), Some(0)); // one a + b
    assert_eq!(parser.parse("aaab"), Some(0)); // multiple a's + b
    assert_eq!(parser.parse("c"), Some(0)); // alternative c
    assert_eq!(parser.parse("d"), None); // no match
}

#[test]
fn complex_pattern_2() {
    let parser = RegexParser::from("(a|b)*c");
    assert_eq!(parser.parse("c"), Some(0)); // empty + c
    assert_eq!(parser.parse("ac"), Some(0)); // a + c
    assert_eq!(parser.parse("bc"), Some(0)); // b + c
    assert_eq!(parser.parse("abac"), Some(0)); // multiple + c
}
