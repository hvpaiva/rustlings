// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

fn main() {
    // You can optionally experiment here.
    println!("10 apples cost {} rustbucks", calculate_price_of_apples(10));
    println!("40 apples cost {} rustbucks", calculate_price_of_apples(40));
    println!("80 apples cost {} rustbucks", calculate_price_of_apples(80));
}

fn calculate_price_of_apples(apples: u16) -> u16 {
    if apples > 40 {
        apples
    } else {
        apples * 2
    }
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
