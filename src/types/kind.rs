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
    Tunnel,
    Floor,
}

impl CellKind {
    pub fn symbol(self) -> char {
        match self {
            CellKind::SoftArmor => '(',
            CellKind::HardArmor => '[',
            CellKind::BluntWeapon => '\\',
            CellKind::EdgedWeapon => '|',
            CellKind::PointedWeapon => '/',
            CellKind::RangedWeapon => '}',
            CellKind::ClosedDoor => '+',
            CellKind::OpenedDoor => '\'',
            CellKind::Wall => '#',
            CellKind::Tunnel => '░',
            CellKind::Floor => '.',
        }
    }
}
