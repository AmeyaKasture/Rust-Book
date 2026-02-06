fn main() {
    let mut orig_vector = vec![String::from("original")];
    println!("Value is {}", orig_vector[0]);
    let new_vector = orig_vector;
    println!("{}", new_vector[0]);
    //new_vector.push(String::from("hi")); cannot use this as the vector as we are transfering ownership in mutable sense
    let first_ref: &Vec<String> = &new_vector;
    // let val = first_ref[0]; This would not work because this contains the string and thus would result in transfer of ownerhsip
    let value = &first_ref[0]; // This works because here we are just borrowing the reference
    // println!("{first_ref[0}"); Cannot work because `{}` expects only simple values for fast print operations and Dereferencing values are not allowed
    println!(
        "Correct way to extract from the first ref where we dereference seperately :{}",
        first_ref[0]
    );
    println!("{value}");
    orig_vector = new_vector;
    // orig_vector.push(String::from("New val is appended"));
    println!("Vector is {:?}", orig_vector);
    //Thus we can see if the ownership is transfered back to the mutable it can be mutable
    //println!("{}",first_ref[0]);cannot use this as this borrowed the new vector stack frame
    //Because new_vector is gone, any references that were pointing into it (like value and first_ref) are immediately considered invalid by the Borrow Checker.
}
