use core::num::NonZeroU64;
use std::iter::successors;

const UNITS: [&str; 10] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SHORT_SCALE: [&str; 8] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
    "sextillion",
];

fn split_digits(n: u64) -> (usize, usize, usize) {
    (
        (n / 100) as usize,
        (n % 100 / 10) as usize,
        (n % 10) as usize,
    )
}

fn convert_triplet_to_phrase(n: u64) -> String {
    let mut words = Vec::new();

    let (hundreds_digit, tens_digit, ones_digit) = split_digits(n);

    if hundreds_digit > 0 {
        words.push(format!("{} hundred", UNITS[hundreds_digit]));
    }

    if tens_digit > 0 || ones_digit > 0 {
        words.push(match (tens_digit, ones_digit) {
            (0, _) => UNITS[ones_digit].to_string(),
            (1, _) => TEENS[ones_digit].to_string(),
            (_, 0) => TENS[tens_digit].to_string(),
            (_, _) => format!("{}-{}", TENS[tens_digit], UNITS[ones_digit]),
        });
    }

    words.join(" ")
}

fn split_number_into_triplets(number: u64) -> Vec<u64> {
    successors(NonZeroU64::new(number), |x| {
        NonZeroU64::new(x.get() / 1_000)
    })
    .map(|x| x.get() % 1_000)
    .collect::<Vec<u64>>()
}

fn number_to_words(number: i64) -> String {
    match number {
        0 => "zero".to_string(),
        _ => split_number_into_triplets(number as u64)
            .iter()
            .zip(SHORT_SCALE.iter())
            .rev()
            .filter_map(|(triplet, scale)| match (triplet, scale) {
                (0, _) => None,
                (_, &"") => Some(convert_triplet_to_phrase(*triplet)),
                (_, _) => Some(convert_triplet_to_phrase(*triplet) + " " + scale),
            })
            .collect::<Vec<String>>()
            .join(" ")
            .to_string(),
    }
}

fn main() {
    println!("{}", number_to_words(10_021));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_data = vec![
            (0, "zero"),
            (1, "one"),
            (9, "nine"),
            (10, "ten"),
            (11, "eleven"),
            (19, "nineteen"),
            (20, "twenty"),
            (21, "twenty-one"),
            (80, "eighty"),
            (90, "ninety"),
            (99, "ninety-nine"),
            (100, "one hundred"),
            (101, "one hundred one"),
            (111, "one hundred eleven"),
            (120, "one hundred twenty"),
            (121, "one hundred twenty-one"),
            (900, "nine hundred"),
            (909, "nine hundred nine"),
            (919, "nine hundred nineteen"),
            (990, "nine hundred ninety"),
            (999, "nine hundred ninety-nine"),
            (1_000, "one thousand"),
            (2_000, "two thousand"),
            (4_000, "four thousand"),
            (5_000, "five thousand"),
            (11_000, "eleven thousand"),
            (21_000, "twenty-one thousand"),
            (999_000, "nine hundred ninety-nine thousand"),
            (999_999, "nine hundred ninety-nine thousand nine hundred ninety-nine"),
            (1_000_000, "one million"),
            (2_000_000, "two million"),
            (4_000_000, "four million"),
            (5_000_000, "five million"),
            (100_100_100, "one hundred million one hundred thousand one hundred"),
            (500_500_500, "five hundred million five hundred thousand five hundred"),
            (606_606_606, "six hundred six million six hundred six thousand six hundred six"),
            (999_000_000, "nine hundred ninety-nine million"),
            (999_000_999, "nine hundred ninety-nine million nine hundred ninety-nine"),
            (999_999_000, "nine hundred ninety-nine million nine hundred ninety-nine thousand"),
            (999_999_999, "nine hundred ninety-nine million nine hundred ninety-nine thousand nine hundred ninety-nine"),
            (1_174_315_110, "one billion one hundred seventy-four million three hundred fifteen thousand one hundred ten"),
            (1_174_315_119, "one billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            (15_174_315_119, "fifteen billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            (35_174_315_119, "thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            (935_174_315_119, "nine hundred thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
            (2_935_174_315_119, "two trillion nine hundred thirty-five billion one hundred seventy-four million three hundred fifteen thousand one hundred nineteen"),
        ];
        for (number, translation) in test_data {
            assert_eq!(super::number_to_words(number), translation);
        }
    }
}
