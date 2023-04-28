use cocoa_purity::map_enum;

#[derive(Debug)]
pub enum Base {
    A,
    B,
    C,
}

#[map_enum(Base)]
pub enum Mapped {
    Nero = A,
    Ada = B,
}

fn main() {
    let mapped = Mapped::Nero;
    let base: Base = mapped.into();
    dbg!(base);
}
