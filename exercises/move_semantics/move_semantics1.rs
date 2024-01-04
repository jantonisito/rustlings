// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    //one possible solution
    //let vec1 = fill_vec(vec0.clone());

    //another possible solution
    let vec1 = fill_vec2(&vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
    
    println!("vec0 is {:?}", vec0);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

//in this case, we are passing a reference to the vector, so we don't need to clone it
fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(88);

    vec
}