mod dicts;

use dicts::{ADJECTIVES, NOUNS};

pub fn print_words() {
    println! {"{}",ADJECTIVES[1000]};
    println! {"{}", NOUNS[1000]};
}

pub fn generate_nickname(_hex: &str) -> &str {
    "nickname"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjective_dictionary() {
        // Checks a few randomly selected adjectives
        let adjective_4 = "Sweeping";
        let adjective_1683 = "Atomic";
        let adjective_4774 = "Satoshi";

        assert_eq!(dicts::ADJECTIVES[4], adjective_4);
        assert_eq!(dicts::ADJECTIVES[1683], adjective_1683);
        assert_eq!(dicts::ADJECTIVES[4774], adjective_4774);
    }

    #[test]
    fn test_nouns_dictionary() {
        // Checks a few randomly selected nouns
        let noun_118 = "Address";
        let noun_6540 = "Null";
        let noun_10937 = "Zombie";

        assert_eq!(dicts::NOUNS[118], noun_118);
        assert_eq!(dicts::NOUNS[6540], noun_6540);
        assert_eq!(dicts::NOUNS[10937], noun_10937);
    }

    #[test]
    fn test_nickname_generator() {
        assert_eq!(generate_nickname("a"), "nickname");
    }
}
