pub fn hello() {
    println!("Hello, world!");
}

pub fn first_ownership_example() {
    let mut nums = vec![1, 2, 3, 4, 5];
    let my_ref = &nums;
    let my_ref_2 = &nums;

    println!("my_ref: {:?}", my_ref);
    println!("my_ref_2: {:?}", my_ref_2);

    println!("my_ref: {:?}", my_ref);
    println!("my_ref_2: {:?}", my_ref_2);

    let my_mut_ref = &mut nums;
    my_mut_ref.push(6);

    println!("my_mut_ref: {:?}", my_mut_ref);

    let my_ref = &nums;
    let my_ref_2 = &nums;

    println!("my_ref: {:?}", my_ref);
    println!("my_ref_2: {:?}", my_ref_2);
}
