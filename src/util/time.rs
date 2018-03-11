pub fn sec_to_str(time: u32) -> String {
    format!("{:02}:{:02}:{:02}", time / 3600, time / 60 % 60, time % 60)
}
