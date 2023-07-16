#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]

// RFC4271 8.1で定義されているEventを表す列挙型の構造体
pub enum Event {
    ManualStart,
    TCPConnectionConfirmed,
}