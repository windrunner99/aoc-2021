mod aoc;

fn main() {
    let d1 = aoc::d01::DayOne::new();
    println!("{}", d1.one());
    println!("{}", d1.two());

    let d2 = aoc::d02::DayTwo::new();
    println!("{}", d2.one());
    println!("{}", d2.two());
}