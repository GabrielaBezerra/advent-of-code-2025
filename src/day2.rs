use std::collections::HashSet;

pub const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
// pub const TEST_INPUT: &str = "2121212118-2121212119";
pub const INPUT: &str = r"959516-995437,389276443-389465477,683-1336,15687-26722,91613-136893,4-18,6736-12582,92850684-93066214,65-101,6868676926-6868700146,535033-570760,826141-957696,365650-534331,1502-2812,309789-352254,79110404-79172400,18286593-18485520,34376-65398,26-63,3333208697-3333457635,202007-307147,1859689-1936942,9959142-10053234,2318919-2420944,5142771457-5142940464,1036065-1206184,46314118-46413048,3367-6093,237-481,591751-793578";

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut matrix: Vec<Vec<i64>> = vec![];
    for range in input.split(",") {
        let mut range_str_vector = range.trim().split("-");
        let a: i64 = range_str_vector.next().unwrap().parse::<i64>().unwrap();
        let b: i64 = range_str_vector.next().unwrap().parse::<i64>().unwrap();
        let range_vector: Vec<i64> = (a..=b).collect();
        matrix.push(range_vector.clone());
    }
    return matrix;
}

pub fn part1(input: &str) {
    let mut invalid_ids: Vec<i64> = vec![];
    let matrix = parse_input(input);
    for range in matrix {
        // println!("{}", range.iter().map(|&e| e.to_string()).collect::<Vec<String>>().join(", "));
        // println!("---");
        for number in range {
            let number_of_digits = number.to_string().chars().count();
            let mut div = 0;
            for _ in 0..number_of_digits {
                div += 1;
            }
            loop {
                if div == 0 {
                    break;
                }
                let left = &number.to_string()[0..div];
                let right = &number.to_string()[div..];
                // println!("{:?} => {:?} == {:?}", number, left, right);
                if left == right {
                    invalid_ids.push(number);
                    // println!("INVALID! {}", number);
                    break;
                }
                div -= 1;
            }
        }
    }
    let total: i64 = invalid_ids.iter().sum();
    println!("sum: {}", total);
}

pub fn part2(input: &str) {
    let mut invalid_ids: HashSet<i64> = vec![].into_iter().collect();
    let matrix = parse_input(input);
    for range in matrix {
        for number in range {
            let number_of_digits = number.to_string().chars().count();
            let mut pivot = 0;
            loop {
                if pivot >= (number_of_digits)/2 {
                    break;
                }
                /*
                232323
                pivot 1
                [23] [23] [23]
                0 1  2 3  4 5

                123123123
                pivot 2
                [123] [123] [123]
                0 1 2 3 4 5 6 7 8
                */
                let mut start = 0;
                let mut split = pivot;
                // println!("{}", number);
                loop {
                    if split + pivot + 1 >= number_of_digits {
                        pivot = number_of_digits/2;
                        break; 
                    }
                    let left = &number.to_string()[start..=split];
                    let right = &number.to_string()[split+1..=split+pivot+1];
                    // println!("left {} == right {}", left, right);
                    if left == right { 
                            if start >= number_of_digits - (pivot + 1) * 2 {
                                // println!("INVALID {} - pivot {}", number, pivot);
                                invalid_ids.insert(number);
                                pivot = number_of_digits/2;
                                break;
                            }
                        start += pivot + 1;
                        split += pivot + 1;
                    } else {
                        pivot += 1;
                        break;
                    }
                    /*
                    1234123412
                    [1234] [1234] [12
                     0123   4567   89

                    111
                    [1] [1] [1]
                     0   1   2
                    111
                    [11] [11] [11]
                    0 1  2 3  4 5
                     */
                }
            }
        }
    }
    let total: i64 = invalid_ids.iter().sum();
    println!("sum: {}", total);
}

