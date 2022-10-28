use md5::{Digest, Md5};

pub fn find_lowest_numeric_md5(input: &'static str) -> i32 {
    let mut number = 0;

    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", input, number).as_bytes());

        let result = hasher.finalize();
        let encoded_result = hex::encode(result);

        if &encoded_result[0..=4] == "000000" {
            return number;
        } else {
            print!("{} ", number);
            number += 1;
        }
    }
}
