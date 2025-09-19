use crate::build_automaton;

// =============================================================================
// EDGE CASES AND BOUNDARY CONDITIONS
// =============================================================================

#[test]
fn empty_string_input() {
    let parser = build_automaton("a*");
    assert_eq!(parser.parse(""), Some(0)); // matches empty string
}

#[test]
fn empty_string_input_no_match() {
    let parser = build_automaton("a");
    assert_eq!(parser.parse(""), None); // requires at least one 'a'
}

#[test]
fn single_character_input() {
    let parser = build_automaton("a");
    assert_eq!(parser.parse("a"), Some(0));
    assert_eq!(parser.parse("b"), None);
}

#[test]
fn pattern_longer_than_input() {
    let parser = build_automaton("hello");
    assert_eq!(parser.parse("hel"), None);
}

#[test]
fn match_at_end_of_string() {
    let parser = build_automaton("end");
    assert_eq!(parser.parse("the end"), Some(4));
}

#[test]
fn multiple_possible_matches() {
    let parser = build_automaton("a");
    assert_eq!(parser.parse("banana"), Some(1)); // should return first match
}