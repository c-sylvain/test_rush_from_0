use the_great_accident::proficiencies::correction::find_banned;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_scrambled() {
    let text = "The S-dragon and the wizards without [the dragons] are here";
    let banned = vec![("dragon", "*"), ("wizard", "X")];
    let (found, corrected) = find_banned(text, &banned);

    assert_eq!(found.len(), 2);
    assert_eq!(found[0], "S-dragon");
    assert_eq!(found[1], "wizards");
    assert_eq!(corrected, "The S-* and the Xs without [the dragons] are here");
}
