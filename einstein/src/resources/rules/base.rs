use crate::resources::manager::Resource;

pub enum TextItem<'a> {
    Text(&'a str),
    Image(Resource),
}
