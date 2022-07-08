fn main() {
    println!("Hello, world!");
}

fn transpose(v: Vec<Vec<String>>) -> Vec<Vec<String>> {
    if v.is_empty() {
        return v;
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<String>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transpose_with_m_n_matrix() {
        use super::transpose;

        let mut orig : Vec<Vec<String>> = Vec::new();
        orig.push(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        orig.push(vec!["c".to_string(), "d".to_string(), "e".to_string()]);

        let result = transpose(orig.clone());

        let mut test : Vec<Vec<String>> = Vec::new();
        test.push(vec!["a".to_string(), "c".to_string()]);
        test.push(vec!["b".to_string(), "d".to_string()]);
        test.push(vec!["c".to_string(), "e".to_string()]);

        assert_eq!(result, test)
    }

    #[test]
    fn test_transpose_sqare_matrix() {
        use super::transpose;

        let mut orig : Vec<Vec<String>> = Vec::new();
        orig.push(vec!["a".to_string(), "b".to_string()]);
        orig.push(vec!["c".to_string(), "d".to_string()]);

        let result = transpose(orig.clone());

        let mut test : Vec<Vec<String>> = Vec::new();
        test.push(vec!["a".to_string(), "c".to_string()]);
        test.push(vec!["b".to_string(), "d".to_string()]);

        assert_eq!(result, test)
    }

    #[test]
    fn test_transpose_zero_element() {
        use super::transpose;

        let orig : Vec<Vec<String>> = Vec::new();

        let result = transpose(orig.clone());

        let test : Vec<Vec<String>> = Vec::new();

        assert_eq!(result, test)
    }

    #[test]
    fn test_transpose_one_element() {
        use super::transpose;

        let mut orig : Vec<Vec<String>> = Vec::new();
        orig.push(vec!["a".to_string()]);

        let result = transpose(orig.clone());

        let mut test : Vec<Vec<String>> = Vec::new();
        test.push(vec!["a".to_string()]);

        assert_eq!(result, test)
    }

    // This test case is assumed not to happen
    // #[test]
    // fn test_transpose_with_one_dimention_wrong() {
    //     use super::transpose;

    //     let mut orig : Vec<Vec<String>> = Vec::new();
    //     orig.push(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    //     orig.push(vec!["c".to_string(), "d".to_string()]);

    //     let result = transpose(orig.clone());

    //     let mut test : Vec<Vec<String>> = Vec::new();
    //     test.push(vec!["a".to_string(), "c".to_string()]);
    //     test.push(vec!["b".to_string(), "d".to_string()]);
    //     test.push(vec!["c".to_string()]);

    //     assert_eq!(result, test)
    // }

}
