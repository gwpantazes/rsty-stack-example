impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        // Convert k to array form.
        let k_array_form = Vec::new();
        let mut k_total = k;
        while k_total > 9 {
            k_array_form.push(k_total % 10);
            k_total /= 10;
        }
        println!("Convert k to array form. {:?}", k_array_form);

        let &more_digits = if num.len() > k_array_form.len() {
            &num
        } else {
            &k_array_form
        };
        println!("More digits: {:?}", &more_digits);

        // Reserve an array with one additional capacity than
        // whichever vector already has the greatest length of digits.
        let possible_number_of_digits: usize = &more_digits.len() + 1;
        let mut result: Vec<i32> = Vec::with_capacity(possible_number_of_digits);
        // Now that both are in array form, add them together digit by digit.
        // Take into account remainders from values over 9.
        let mut remainder: Option<i32> = None;

        // for (i, value) in num.iter().enumerate() {
        //     // Compute the power of ten to apply based on the position
        //     // (instead of cloning and reversing the num vector).
        //     let power: u32 = num.len() as u32 - 1 - i as u32;
        //     total += value * 10i32.pow(power);
        // }

        // let mut new_num = Vec::new();
        // while total > 9 {
        //     new_num.push(total % 10);
        //     total /= 10;
        // }
        // new_num.push(total);
        // new_num.reverse();
        // new_num

        num
    }
}
