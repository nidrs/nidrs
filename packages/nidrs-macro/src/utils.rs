pub fn merge_vec<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    result.extend(a);
    result.extend(b);
    result
}
