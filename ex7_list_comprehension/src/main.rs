fn main() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let newVec: Vec<&i32> = vector.iter().skip(1).step_by(2).collect();
    println!("{:?}, every second value {:?}", vector, newVec)
}