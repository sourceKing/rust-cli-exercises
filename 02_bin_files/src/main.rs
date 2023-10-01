mod mod1;
mod mod2;
mod mod3_sub;
use mod3_sub::mod3;

fn main() {
    let sum = mod2::fn2() + mod3::fn3();
    println!("Total value is: {}", sum);
}
