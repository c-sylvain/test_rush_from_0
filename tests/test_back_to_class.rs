use the_great_accident::proficiencies::back_to_class::compute;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_multiple_expressions() {
    let expr1 = "2 + 3 * 4";
    let expr2 = "10 - 2 + 5 * 2";
    let expr3 = "100 % 7 + 3";

    assert_eq!(compute(expr1), 14);
    assert_eq!(compute(expr2), 18);
    assert_eq!(compute(expr3), 5);
}
