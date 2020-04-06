// this code also cannot compile, even though
// serde_json is not used here explicitly
#[cfg(test)]
mod test {
    #[test]
    fn into_iter() {
        assert_eq!(10, vec![1, 2, 3, 4].iter().sum());
    }
}