fn sum_array_up(arr: &[u32]) -> Option<u32> {
    let mut res: Option<u32> = Some(0);
    for &item in arr {
        res = res.and_then(|i| i.checked_add(item));
        if res.is_none() {
            break;
        }
    }
    res
}

pub fn test_p9() {
    println!("\n############ Practice 9 Start! ############\n");
    let arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arr2 = [1, 2, 3, u32::max_value(), 4];
    println!("arr1{:?}的和是{:?}\narr2{:?}的和是{:?}",
                arr1, sum_array_up(&arr1[..]), arr2, sum_array_up(&arr2[..]));
    println!("\n############ Practice 9 End! ############");
}