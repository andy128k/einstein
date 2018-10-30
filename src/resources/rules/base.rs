pub enum TextItem<'a> {
    Text(&'a str),
    Image(&'a str, &'a [u8]),
}
