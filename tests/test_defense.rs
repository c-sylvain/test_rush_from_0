use the_great_accident::proficiencies::defense::cursed_reverse;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_basic_reversal() {
    let mut test = String::from("Hello, world! How are you?");
    cursed_reverse(&mut test);
    assert_eq!(test, "olleH,dlrow !uoy era woH ?");
}

