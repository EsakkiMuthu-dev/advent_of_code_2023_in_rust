use crate::days::day01;

#[test]
fn test_single_number()
{
    let result  = day01::get_number_from_str("treb7uchet".to_string());
    assert_eq!(result,"77")
}

#[test]
fn test_two_numbers()
{
    let result = day01::get_number_from_str("1abc2".to_string());
    assert_eq!(result,"12")
}
#[test]
fn test_more_than_two_numbers()
{
    let result = day01::get_number_from_str("a1b2c3d4e5f".to_string());
    assert_eq!(result,"15")
}