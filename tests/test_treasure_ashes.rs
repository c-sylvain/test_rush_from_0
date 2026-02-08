use the_great_accident::fundamentals::treasure_ashes::recover_manuscript;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_season_2() {
    let corrupted = "Tyщgrener's bцsss sss 🎵 arrфrdфnnιιι sфnnggsss ⚔️";
    let result = recover_manuscript(corrupted);
    assert_eq!(result, "Tygren's bad");
}
