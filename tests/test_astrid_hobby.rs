use the_great_accident::fundamentals::astrid_hobby::swords_analysis;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_songs_of_war() {
    let test_sheet = "Ingressus 沃尔塔里斯 👑";
    let (content, byte_len, char_len, ratio) = swords_analysis(test_sheet);
    
    assert_eq!(content, "Ingressus 沃尔塔里斯 👑");
    assert_eq!(byte_len, 30);
    assert_eq!(char_len, 17);
    assert_eq!(ratio, 17.0 / 3.0);
}
