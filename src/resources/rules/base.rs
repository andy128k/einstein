pub enum TextItem<'a> {
    Text(&'a str),
    Image(&'a [u8]),
}
