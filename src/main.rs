
#[allow(dead_code)] // allows to turn off warnings for unused imports 
fn double(x: i32) -> i32 {
    x * 2
}
#[allow(unused_variables, unused_mut)]
fn quadruple(x: i32)-> i32 {
  2 * (x*2)
}     
fn main() {
    println!("Answer: {}", quadruple(5));
}