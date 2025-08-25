fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let mut return_vec: Vec<i32> = vec![0];
    return_vec = iterate_through_nums(&nums, target);
    return_vec
}

fn iterate_through_nums(nums: &Vec<i32>, target: i32) -> Vec<i32>{
    let nums_iter = nums.iter();
    let mut has_happened_already = false;
    let mut return_vec= vec![0];

    for (i, num1) in nums_iter.clone().enumerate(){
        for (x, num2) in nums_iter.clone().enumerate(){
            if i != x{
                if !has_happened_already{
                    if num1 + num2 == target{
                        has_happened_already = true;
                        println!("{}, {}",num1, num2);
                        return_vec = vec![i.clone() as i32, x.clone() as i32];
                        return return_vec;
                    }
                }
            }
        }
    }
    return_vec
}

fn main(){
    let mut nums = vec![15,2,11,7];
    let target = 9;
    let sum_result = two_sum(nums, target);
    println!("{}, {}, {}", sum_result[0], sum_result[1], target);

    nums = vec![2,5,5,11]; //should return [1,2] but returns [1,1]
    let target = 10;
    let sum_result = two_sum(nums, target);
    println!("{}, {}, {}", sum_result[0], sum_result[1], target);

    nums = vec![3,3];
    let target = 6;
    let sum_result = two_sum(nums, target);
    println!("{}, {}, {}", sum_result[0], sum_result[1], target);

    nums = vec![3,2,3];
    let target = 6;
    let sum_result = two_sum(nums, target);
    println!("{}, {}, {}", sum_result[0], sum_result[1], target);

    nums = vec![-3,4,3,90];
    let target = 0;
    let sum_result = two_sum(nums, target);
    println!("{}, {}, {}", sum_result[0], sum_result[1], target);
}