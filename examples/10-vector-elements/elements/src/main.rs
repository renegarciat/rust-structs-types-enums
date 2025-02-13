
fn add_new(v: &mut Vec<i32>, element: i32) {
    v.insert(0, element); //Insert the element at the beginning.
    v.push(element); //Insert the element at the end.
}

fn append_two_vectors(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.append(v2);
}

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(&more_numbers);
    println!("{:?}", v);
    println!("{:?}",more_numbers);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    // Insert an item at the beginning and the end.
    add_new(&mut v, 99);
    println!("{:?}", v);

    // Append v2 to v
    let mut v2 = vec![-1,-1,-1];
    append_two_vectors(&mut v, &mut v2);
    println!("{:?}", v);
}
