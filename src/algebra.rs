pub enum Nothing {} // !

pub enum OneOf<T1, T2, T3> {
    v1(T1),
    v2(T2),
    v3(T3),
}
