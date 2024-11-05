use std::io::{BufRead, BufReader};
use std::fs::File;


pub fn day1()
{
    let file =File::open("/Users/esakki-20378/Documents/GitHub/advent_of_code_2023/inputs/day01.txt").expect("Cannot open the file");
    let reader = BufReader::new(file);

    let mut  total_sum =0;
    for line in reader.lines(){
        let line = line.unwrap();
        let num:i32 = get_number_from_str(line.clone()).parse().expect("Not an Number");
        println!(" For this String : {} got This number {} ",line,num);
        total_sum+=num;
    }

    println!("{total_sum}")
}

pub fn get_number_from_str( line:String) -> String {
    let mut numbers: Vec<i8> = line.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect();
    let mut len = numbers.len();
    if len == 1
    {
        let num = numbers.get_mut(0).unwrap().to_string();
        return format!("{}{}", num, num);
    }
    let num1 = numbers.get_mut(0).unwrap().to_string();
    let num2 = numbers.get_mut(len - 1).unwrap().to_string();
    return format!("{}{}", num1, num2);
}

