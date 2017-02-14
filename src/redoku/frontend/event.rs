use value::Value;

pub enum Event {
    MenuInit,
    MenuX,
    MenuY,
    MenuZ,
    Redoku,
    RedokuCursor(u8),
    RedokuGridValue(u8, Option<Value>),
}
