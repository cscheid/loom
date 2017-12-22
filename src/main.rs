use vector::V;

mod vector;

fn main() {
    let v1 = V::new(1.0, 0.0, 0.0);
    let v2 = V::new(0.0, 1.0, 0.0);
    println!("v1 is {:?}", v1);
    println!("v1 + v2 is {:?}", &v1 + &v2); // this is a little goofy, but ok
    println!("v1 - v2 is {:?}", &v1 - &v2);
    println!("v1 * v1 is {:?}", &v1 * &v2);
}
