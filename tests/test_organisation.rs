use the_great_accident::proficiencies::organisation::fix_database;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_guess_why() {
    let raw_data = "Draco|Dragon\n\
        Wyvern,Wyvern\n\
        Dragon: Aurora Type: Unknown\n\
        {Skyra}[Skydancer]\n\
        Unnamed entry\n\
        {}  [ {}[{}}[]";

    let result = fix_database(raw_data);
    assert_eq!(result, "Draco [new]|DRAGON\n\
                    Wyvern [new]|WYVERN\n\
                    Aurora [new]|AURORA\n\
                    Skyra [new]|SKYDANCER\n\
                    }  [ {}[{}]|{}[{}");
}
