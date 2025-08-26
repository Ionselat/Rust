pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative(){
        return false;
    }
    
    let a = x.to_string();
    let return_vec = deconstruct_string_to_vector(a);
    let mut is_reverse = false;
    let mut vec_copy = return_vec.clone();
    println!("{:#?}", return_vec);
    vec_copy.reverse();
    for i in 0..vec_copy.len(){
        if vec_copy[i] != return_vec[i]{
            return false;
        }
    }
    true
}

fn deconstruct_string_to_vector(x: String) -> Vec<char>{
    let mut vec = vec![];
    for i in x.chars(){
        vec.push(i);
    }    
    vec
}

fn main() {
    let result = is_palindrome(121);
    println!("{result}");

    let result = is_palindrome(-121);
    println!("{result}");
}
