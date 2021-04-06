use std::collections::HashMap;

/// Moves all the Labels starting at `starting_point`(exclusive) by `offset`
pub fn move_labels(labels: &mut HashMap<String, u32>, starting_point: u32, offset: u32) {
    for (_, value) in labels.iter_mut() {
        if *value > starting_point {
            *value += offset;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_all() {
        let mut inital = HashMap::new();
        inital.insert("test".to_owned(), 1);
        inital.insert("test2".to_owned(), 3);

        let mut expected = HashMap::new();
        expected.insert("test".to_owned(), 3);
        expected.insert("test2".to_owned(), 5);

        move_labels(&mut inital, 0, 2);

        assert_eq!(expected, inital);
    }

    #[test]
    fn move_partial() {
        let mut inital = HashMap::new();
        inital.insert("test".to_owned(), 0);
        inital.insert("test2".to_owned(), 3);

        let mut expected = HashMap::new();
        expected.insert("test".to_owned(), 0);
        expected.insert("test2".to_owned(), 6);

        move_labels(&mut inital, 1, 3);

        assert_eq!(expected, inital);
    }

    #[test]
    fn move_partial_not_inclusive() {
        let mut inital = HashMap::new();
        inital.insert("test".to_owned(), 1);
        inital.insert("test2".to_owned(), 3);

        let mut expected = HashMap::new();
        expected.insert("test".to_owned(), 1);
        expected.insert("test2".to_owned(), 6);

        move_labels(&mut inital, 1, 3);

        assert_eq!(expected, inital);
    }
}
