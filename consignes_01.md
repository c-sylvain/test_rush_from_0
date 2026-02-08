The Great Accident 1.0.2
Strings, String Slices, String Iterators, Standard String Methods, Parsing

# Guidelines


## Repository structure

At the end, your git repo must follow this architecture:
epita-prepa-computer-science-prog-207-p-04-2029-firstname.lastname
├── the_great_accident
│   ├── src
│   │   ├── fundamentals
│   │   │   ├── astrid_hobby.rs
│   │   │   ├── cautious_glory.rs
│   │   │   ├── dragcon.rs
│   │   │   ├── dragon_scale.rs
│   │   │   ├── intruder.rs
│   │   │   ├── mod.rs
│   │   │   ├── population_expansion.rs
│   │   │   └── treasure_ashes.rs
│   │   ├── proficiencies
│   │   │   ├── back_to_class.rs
│   │   │   ├── correction.rs
│   │   │   ├── defense.rs
│   │   │   ├── mod.rs
│   │   │   └── organisation.rs
│   │   └── lib.rs
│   └── Cargo.toml
├── .gitignore
└── README
Copy


The .gitignore file is mandatory.
Remove all personal tests from your code, except those from the Tests folder.
The given prototypes must be strictly respected.
The code MUST compile! Otherwise you will not receive a grade.


### .gitignore example

debug/
target/
Cargo.lock

.idea/
*~
*.DotSettings.user

This need to be setup before the first submission!

# Introduction

## Information

Throughout the subject, you are given code examples to help you understand the instructions.
To relieve your pain of copy-pasting and recompiling a main function everytime, we have provided you with test files containing the same examples

This given "testsuite" is only a MINIMUM.
You SHOULD write your own tests in those files.


Rust possesses a rich standard library especially about strings. You should always check on your own the documentation : https://doc.rust-lang.org/std/string/struct.String.html for convenient functions/methods you could use.

Useful iterators for string manipulation (and sufficient for this practical) include chars(), split() and its variants like split_whitespace(), lines(), and char_indices().

## Dangers

In Rust, many string functions use types like Option and Result, which are future practicals for you.

To keep this practical about strings, the following are forbidden unless an exercise explicitly authorizes them:
1. Functions that return Option<T> or Result<T, E> types:
Examples: pop(), strip_prefix(), strip_suffix(), parse(), etc.
2. The specific following functions (to prevent you from using bytes, vectors, or raw pointers>):
as_bytes(), into_bytes(), as_mut_vec()
as_ptr(), as_mut_ptr()
into_raw_parts(), from_raw_parts()

Individual exercises may have their own restrictions or rules.
When a specific exercise authorizes a function (like find()), use it as described in the exercise, without needing to understand how it works.

# Fundamentals - Restoration

## Exercice 1
fundamentals/cautious_glory.rs

### todo
Concatenate all valid strings from the given vector data of tuples.
A string is considered valid if its associated boolean (in the same tuple) is true.
The resulting output must separate all valid strings with the ';' character.

### Dangers
You are not allowed to use the join() function.

### Prototype(s)
pub fn valid_sheets(data: Vec<(&str, bool)>) -> String {}

### Code example(s)
println!("{}", valid_sheets(vec![
    ("The secret to happiness is freedom", true),
    ("And the secret to freedom is courage", false),
    ("Courage is not the absence of fear", true),
    ("But the triumph over it", true),
]));

### Output
The secret to happiness is freedom;Courage is not the absence of fear;But the triumph over it

## Exercice 2
fundamentals/astrid_hobby.rs
### Todo
Taking a string in parameter, retrieve the following information about it in a tuple:
Its content as a slice (This is also called "borrowing").
The number of bytes it is composed of: bytes_nbr.
The number of characters it is composed of: chars_nbr.
And the float ratio chars_nbr/word_nbr, or 0 if no words.
The returned tuple should be of the format (<content>, <bytes_nbr>, <chars_nbr>, <ratio>).
A word is any continous sequence of non-whitespace characters.

### Prototype(s)
pub fn swords_analysis(sheet: &str) -> (&str, usize, usize, f32) {}


### Code example(s)
let test_sheet = "Ingressus 沃尔塔里斯 👑";
let (content, byte_len, char_len, ratio) = swords_analysis(test_sheet);

println!("Content: \"{}\"", content);
println!("Byte length: {}", byte_len);
println!("Char count: {}", char_len);
println!("Ratio (chars/words): {}", ratio);

### Output
Content: "Ingressus 沃尔塔里斯 👑"
Byte length: 30
Char count: 17
Ratio (chars/words): 5.6666665

## Exercice 3 

### Todo
Return a clone of the given string with only the ascii characters kept.
You must also remove any duplicated ascii characters, whether they're following each other or not.

### Hint(s)
You could take a look at the contains() function.
You are also encouraged to search for ascii-related functions in the Rust documentation that could ease your work.

### Danger(s)
You are not allowed to use retain().

### Prototype(s)
pub fn recover_manuscript(corrupted: &str) -> String {}


### Code example(s)
let test = "Tyщgrener's bцsss sss 🎵 arrфrdфnnιιι sфnnggsss ⚔️";
println!("{}", recover_manuscript(test));


### Output
Tygren's bad

## Exercice 4
fundamentals/dragon_scale.rs
### todo
Count the number of dragons whose power level falls into the given range (both bounds included).
The string is a list of "dragon entries" separated by the ';' character.

A "dragon entry" is a list of <key>:<value> pairs, separated by a comma, the whole encapsulated in "{}" braces.
Example: "{name:"Toothless",retro:"or not",power:3,another:"empty",power:15}"

One of those is the pair power:<nbr>.
Since the keys are not unique, you should only take the first power.

### Information
For this exercise only, you may need (and are allowed) to use the find() function as described below.

Writing "<String>.find(<String or char>).unwrap()":
Returns the byte index of the first occurrence of the element if found.
Panics if the element has not been found (similar to dereferencing NULL in C).
It is your duty to ensure the element is contained in the searched string before that.

### Hint(s)
Once again, you could find useful functions in the documentation to, say, check if a character is a whitespace.
We also recommend len_utf8() to know the byte size of a character which somewhat reconciles how Rust separates the byte and character versions of a string.

### Prototype(s)
pub fn dragon_powerscale(record: &str, min_power: u32, max_power: u32) -> usize {}


### Code example(s)
let test = "{name:Toothless,power:1000};{name:Hookfang,power:650};{name:Stormfly,power:700};{name:Meatlug,power:500};\
            {name:Barf,power:400};{name:Belch,power:400};{name:Cloudjumper,power:950}";
let count = dragon_powerscale(test, 600, 1000);

println!("Result: {}", count);
Copy

### Output
Result: 4

## Exercice 4



## Exercice 4



## Exercice 4



## Exercice 4



## Exercice 4
