mod day1;
mod day2;
mod day3;

fn main() { 
   println!("\n=== DAY 1 - part 1 ===");
   day1::part1(day1::TEST_INPUT);
   day1::part1(day1::INPUT);
   println!("\n=== DAY 1 - part 2 ===");
   day1::part2(day1::TEST_INPUT);
   day1::part2(day1::INPUT);

   println!("\n=== DAY 2 - part 1 ===");
   day2::part1(day2::TEST_INPUT);
   day2::part1(day2::INPUT);
   println!("\n=== DAY 2 - part 2 ===");
   day2::part2(day2::TEST_INPUT);
   day2::part2(day2::INPUT);

   println!("\n=== DAY 3 - part 1 ===");
   day3::part1(day3::TEST_INPUT);
   day3::part1(day3::INPUT);
   println!("\n=== DAY 3 - part 2 ===");
   day3::part2(day3::TEST_INPUT);
   day3::part2(day3::INPUT);
}
