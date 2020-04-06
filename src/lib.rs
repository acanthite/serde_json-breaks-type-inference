// works just fine without this line
use serde_json;

#[cfg(test)]
mod test {
    use std::collections::LinkedList;

    #[test]
    fn into_iter() {
        assert_eq!(10, vec![1, 2, 3, 4].iter().sum());
    }

    #[test]
    fn other_iter() {
        let mut linked = LinkedList::new();
        linked.push_back(1);
        linked.push_back(2);
        linked.push_back(3);
        linked.push_back(4);
        assert_eq!(10, linked.iter().sum())
    }
}