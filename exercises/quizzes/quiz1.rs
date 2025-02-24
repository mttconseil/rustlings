// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn calculate_price_of_apples(nb:i32) -> i32 {
    if nb<41 {
        nb*2
    } else {
        nb
    }
}

fn main() {
    // You can optionally experiment here.
    println!("le total pour {} pomme(s) est {}",2,calculate_price_of_apples(2));
    println!("le total pour {} pomme(s) est {}",20,calculate_price_of_apples(20));
    println!("le total pour {} pomme(s) est {}",40,calculate_price_of_apples(40));
    println!("le total pour {} pomme(s) est {}",41,calculate_price_of_apples(41));
    println!("le total pour {} pomme(s) est {}",50,calculate_price_of_apples(50));
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
