#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellAccess {
    Takeable,
    Static,
    Impassable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellKind {
    SoftArmor,
    HardArmor,
    BluntWeapon,
    EdgedWeapon,
    PointedWeapon,
    RangedWeapon,
    ClosedDoor,
    OpenedDoor,
    Wall,
}
