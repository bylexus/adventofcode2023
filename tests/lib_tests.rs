use adventofcode2023::lines_to_numbers;


#[test]
fn test_lines_to_numbers() {
    let input: Vec<String> = Vec::from([
        "2".to_string(),
        "12345678".to_string(),
        "    12345678".to_string(),
        "    87654321    ".to_string(),
        "42    ".to_string(),
    ]);
    assert_eq!(
        lines_to_numbers(&input),
        vec![2, 12345678, 12345678, 87654321, 42]
    );
}
