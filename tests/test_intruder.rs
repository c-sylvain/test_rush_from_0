use the_great_accident::fundamentals::intruder::verify_identity;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_identity_verification() {
    let list = "sslhtt|ToothBroken\n\
            gnfkh|Hookfang\n\
            sslhtt|Toothless|correct\n\
            lfmrts|Stormfly";
    let result = verify_identity("Toothless", list);
    assert_eq!(result, "sslhtt|Toothless|correct");
}
