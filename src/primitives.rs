use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub nat_align: u8,
    pub size: u8,
}

#[rustfmt::skip]
pub static TYPE_INFO: LazyLock<HashMap<&'static str, TypeInfo>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    // Integers
    m.insert("u8", TypeInfo { nat_align: 1, size: 1 });
    m.insert("i8", TypeInfo { nat_align: 1, size: 1 });
    m.insert("u16", TypeInfo { nat_align: 2, size: 2 });
    m.insert("i16", TypeInfo { nat_align: 2, size: 2 });
    m.insert("u32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("i32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("u64", TypeInfo { nat_align: 8, size: 8 });
    m.insert("i64", TypeInfo { nat_align: 8, size: 8 });
    // Floating point
    m.insert("f32", TypeInfo { nat_align: 4, size: 4 });
    m.insert("f64", TypeInfo { nat_align: 8, size: 8 });
    // Others
    m.insert("bool", TypeInfo { nat_align: 1, size: 1 });
    m.insert("char", TypeInfo { nat_align: 4, size: 4 }); 
    m.insert("String", TypeInfo { nat_align: 1, size: 3 });
    m
});
