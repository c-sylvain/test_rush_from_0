use the_great_accident::fundamentals::population_expansion::register_newcomer;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_basic_main_character() {
    let test = "name:Toothless, power:1, age:23";
    let result = register_newcomer(test);
    assert_eq!(result, "Toothless | POWER [1] | AGE [23] YEARS");
}
