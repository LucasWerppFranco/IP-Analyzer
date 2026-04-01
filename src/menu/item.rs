pub struct MenuItem {
    pub label: String,
    pub action: fn() -> MenuAction,
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Continue,
    Exit,
}
