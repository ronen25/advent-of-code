mod hashfind;

pub use crate::hashfind::find_lowest_numeric_md5;

fn main() {
    let number = find_lowest_numeric_md5("yzbqklnj");
    println!("The number is: {}", number);
}
