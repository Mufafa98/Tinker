use crate::build_automaton;

// =============================================================================
// OR OPERATOR TESTS
// =============================================================================

#[test]
fn or_match_first_alternative() {
    let parser = build_automaton("a|b");
    assert_eq!(parser.parse("abaaaa"), Some(0));
}

#[test]
fn or_match_second_alternative() {
    let parser = build_automaton("a|b");
    assert_eq!(parser.parse("baaaa"), Some(0));
}

#[test]
fn or_match_later_in_string() {
    let parser = build_automaton("x|y");
    assert_eq!(parser.parse("abxdef"), Some(2));
}

#[test]
fn or_no_match_either() {
    let parser = build_automaton("x|y");
    assert_eq!(parser.parse("abcdef"), None);
}

#[test]
fn or_multiple_alternatives() {
    let parser = build_automaton("a|b|c");
    assert_eq!(parser.parse("cdef"), Some(0));
    assert_eq!(parser.parse("bcdef"), Some(0));
    assert_eq!(parser.parse("xyzc"), Some(3));
}