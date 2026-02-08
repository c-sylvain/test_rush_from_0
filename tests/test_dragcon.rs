use the_great_accident::fundamentals::dragcon::decode_message;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_mixed_feelings() {
    let test = "3*dragon 2*(fly 1*sky)";
    let result = decode_message(test);
    assert_eq!(result, "dragondragondragon fly skyfly sky");
}
