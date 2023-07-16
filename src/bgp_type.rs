#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]

// u16を保持するデータ型．AS番号を保持
pub struct AutonomousSystemNumber(u16);

// ASNumber型の値からu16型の値へ変換
impl From<AutonomousSystemNumber> for u16 {
    fn from(as_number: AutonomousSystemNumber) -> u16 {
        as_number.0
    }
}

// u16型の値からASNumber型の値へ変換
impl From<u16> for AutonomousSystemNumber {
    fn from(as_number: u16) -> Self {
        Self(as_number)
    }
}