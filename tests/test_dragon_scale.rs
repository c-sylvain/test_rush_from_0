use the_great_accident::fundamentals::dragon_scale::dragon_powerscale;

// These tests are given to students as examples tests and meant to be expanded by them.

#[test]
fn test_basic_range() {
    let test = "{name:Toothless,power:1000};{name:Hookfang,power:650};{name:Stormfly,power:700};{name:Meatlug,power:500};\
                {name:Barf,power:400};{name:Belch,power:400};{name:Cloudjumper,power:950}";
    let count = dragon_powerscale(test, 600, 1000);                    
    assert_eq!(count, 4);
}
