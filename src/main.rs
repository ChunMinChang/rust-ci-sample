extern crate rust_ci_sample;

fn main() {
    let x: i32 = -314;
    let abs: i32 = rust_ci_sample::utils::get_abs(x);
    println!("abs of {} is {}", x, abs);
}
