fn get_symbols(mut num: u8, radix: u8, capacity: u8, len: usize) -> Vec<u8> {
    if num < capacity {
        let mut digits = vec![0; len];
        for index in (0..len).rev() {
            digits[index] = num % radix;
            num /= radix;
        }
        digits
    } else {
        get_symbols(num - capacity, radix, capacity * radix, len + 1)
    }
}

pub fn num_to_str(num: u8, alphabet: &[&str]) -> String {
    let radix = alphabet.len() as u8;
    let digits = get_symbols(num, radix, radix, 1);
    let mut result = String::new();
    for digit in digits {
        result.push_str(alphabet[digit as usize]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_str() {
        let alphabet = &["X", "Y", "Z"];
        assert_eq!(num_to_str(0, alphabet), "X");
        assert_eq!(num_to_str(1, alphabet), "Y");
        assert_eq!(num_to_str(2, alphabet), "Z");
        assert_eq!(num_to_str(3, alphabet), "XX");
        assert_eq!(num_to_str(4, alphabet), "XY");
        assert_eq!(num_to_str(5, alphabet), "XZ");
        assert_eq!(num_to_str(6, alphabet), "YX");
        assert_eq!(num_to_str(7, alphabet), "YY");
        assert_eq!(num_to_str(8, alphabet), "YZ");
        assert_eq!(num_to_str(9, alphabet), "ZX");
        assert_eq!(num_to_str(10, alphabet), "ZY");
        assert_eq!(num_to_str(11, alphabet), "ZZ");
        assert_eq!(num_to_str(12, alphabet), "XXX");
        assert_eq!(num_to_str(13, alphabet), "XXY");
        assert_eq!(num_to_str(14, alphabet), "XXZ");
        assert_eq!(num_to_str(15, alphabet), "XYX");
        assert_eq!(num_to_str(16, alphabet), "XYY");
        assert_eq!(num_to_str(17, alphabet), "XYZ");
        assert_eq!(num_to_str(18, alphabet), "XZX");
        assert_eq!(num_to_str(19, alphabet), "XZY");
        assert_eq!(num_to_str(20, alphabet), "XZZ");
        assert_eq!(num_to_str(21, alphabet), "YXX");
        assert_eq!(num_to_str(22, alphabet), "YXY");
    }
}
