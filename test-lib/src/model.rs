#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person() {
        let person1 = Person {
            name: "Alice".to_string(),
        };
        let person2 = Person {
            name: "Alice".to_string(),
        };
        assert!(person.name == "Alice");
        assert_eq!(person1, person2);

        let person3 = Person {
            name: "Bob".to_string(),
        };

        assert_ne!(person1, person3);
    }
}
