#![feature(plugin)]
#![plugin(path_macro)]

macro_rules! some_macro {
    () => ("Hello.")
}

fn main() {
    let with_path = m_path!(some_macro);
    let with_path_work_around = m_path_wa!(some_macro);
    let with_ident = m_ident!(some_macro);
}
