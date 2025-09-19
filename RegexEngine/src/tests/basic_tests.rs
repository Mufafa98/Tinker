use crate::build_automaton;

// =============================================================================
// BASIC SINGLE CHARACTER TESTS
// =============================================================================

#[test]
fn single_letter_match_first() {
    let parser = build_automaton("a");
    assert_eq!(parser.parse("abaaaa"), Some(0));
}

#[test]
fn single_letter_match_middle() {
    let parser = build_automaton("b");
    assert_eq!(parser.parse("abaaaa"), Some(1));
}

#[test]
fn single_letter_no_match() {
    let parser = build_automaton("z");
    assert_eq!(parser.parse("abaaaa"), None);
}
