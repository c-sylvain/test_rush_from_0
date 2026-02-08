use the_great_accident::fundamentals::cautious_glory::valid_sheets;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_quotes() {
    let data = vec![
        ("The secret to happiness is freedom", true),
        ("And the secret to freedom is courage", false),
        ("Courage is not the absence of fear", true),
        ("But the triumph over it", true),
    ];
    let result = valid_sheets(data);
    assert_eq!(result, "The secret to happiness is freedom;Courage is not the absence of fear;But the triumph over it");
}
