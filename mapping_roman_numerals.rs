#[derive(Clone)]
struct RomanNumeral {
    string: String,
    value: i32,
}

impl RomanNumeral{
    fn new(string: String, value: i32) -> RomanNumeral{
        RomanNumeral { string, value}
    }
}

fn convert_string(mut string: String) -> i32 {
    let rn40 = RomanNumeral::new(
        "IX".to_string(), 9);
    let rn30 = RomanNumeral::new(
        "XL".to_string(), 40);
    let rn20 = RomanNumeral::new(
        "XC".to_string(), 90);
    let rn10 = RomanNumeral::new(
        "CD".to_string(), 400);
    let rn01 = RomanNumeral::new(
        "CM".to_string(), 900);
    let rn50 = RomanNumeral::new(
        "IV".to_string(), 4);
    let rn60 = RomanNumeral::new(
        "M".to_string(), 1000);
    let rn70 = RomanNumeral::new(
        "D".to_string(), 500);
    let rn80 = RomanNumeral::new(
        "C".to_string(), 100);
    let rn90 = RomanNumeral::new(
        "L".to_string(), 50);
    let rn100 = RomanNumeral::new(
        "X".to_string(), 10);
    let rn110 = RomanNumeral::new(
        "V".to_string(), 5);
    let rn120 = RomanNumeral::new(
        "I".to_string(), 1);
    let mut roman_vec: Vec<RomanNumeral> = Vec::new();
    roman_vec.push(rn01);
    roman_vec.push(rn10);
    roman_vec.push(rn20);
    roman_vec.push(rn30);
    roman_vec.push(rn40);
    roman_vec.push(rn50);
    roman_vec.push(rn60);
    roman_vec.push(rn70);
    roman_vec.push(rn80);
    roman_vec.push(rn90);
    roman_vec.push(rn100);
    roman_vec.push(rn110);
    roman_vec.push(rn120);

    let mut running_sum: i32 = 0;
    let mut new_string = "".to_string();
    let mut running_string = "".to_string();

    for rn in roman_vec.clone(){
        loop {
            let instance = string.find(&rn.string);
            match instance{
                Some(value) => {
                    // println!("There is a {} to process", &rn.string);
                    string = string.replacen(&rn.string, "", 1);
                    for (i, cha) in rn.string.chars().enumerate(){
                        running_string.push(cha);
                    }
                }
                None => {
                    // println!("No instances of {}", &rn.string);
                    break;
                    }
            } 
            running_sum += rn.value;
        }
    }
    // println!("{} processed, Sum is {}",running_string, running_sum);
    running_sum
}

fn main(){
    let test_string = "XXIV".to_string();
    let return_value = convert_string(test_string);
    println!("{}", return_value);

    let test_string = "CCXXIV".to_string();
    let return_value = convert_string(test_string);
    println!("{}", return_value);
}
