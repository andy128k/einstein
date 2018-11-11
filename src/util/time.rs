pub fn sec_to_str(time: u32) -> String {
    format!("{:02}:{:02}:{:02}", time / 3600, time / 60 % 60, time % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sec_to_str() {
        assert_eq!(&sec_to_str(0), "00:00:00");
        assert_eq!(&sec_to_str(59), "00:00:59");
        assert_eq!(&sec_to_str(60), "00:01:00");
        assert_eq!(&sec_to_str(150), "00:02:30");
        assert_eq!(&sec_to_str(3599), "00:59:59");
        assert_eq!(&sec_to_str(3600), "01:00:00");
        assert_eq!(&sec_to_str(60 * 60 * 60), "60:00:00");
        assert_eq!(&sec_to_str(99 * 60 * 60), "99:00:00");
        assert_eq!(&sec_to_str(100 * 60 * 60 - 1), "99:59:59");
        assert_eq!(&sec_to_str(100 * 60 * 60), "100:00:00");
        assert_eq!(&sec_to_str(100 * 60 * 60 + 1), "100:00:01");
    }
}
