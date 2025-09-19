use crate::build_automaton;

// =============================================================================
// CONCATENATION TESTS
// =============================================================================

#[test]
fn concatenation_two_chars() {
    let parser = build_automaton("ab");
    assert_eq!(parser.parse("abcd"), Some(0));
    assert_eq!(parser.parse("xabcd"), Some(1));
    assert_eq!(parser.parse("acbd"), None);
}

#[test]
fn concatenation_three_chars() {
    let parser = build_automaton("abc");
    assert_eq!(parser.parse("abcdef"), Some(0));
    assert_eq!(parser.parse("xabcdef"), Some(1));
    assert_eq!(parser.parse("abxc"), None);
}

#[test]
fn concatenation_longer_pattern() {
    let parser = build_automaton("hello");
    assert_eq!(parser.parse("hello world"), Some(0));
    assert_eq!(parser.parse("say hello world"), Some(4));
    assert_eq!(parser.parse("helo world"), None);
}