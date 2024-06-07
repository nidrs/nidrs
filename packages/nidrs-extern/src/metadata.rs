#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultPrefix {
    Disabled,
    Enabled,
}

impl DefaultPrefix {
    pub fn as_bool(&self) -> bool {
        match self {
            DefaultPrefix::Enabled => true,
            DefaultPrefix::Disabled => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Global {
    Enabled,
    Disabled,
}

impl Global {
    pub fn as_bool(&self) -> bool {
        match self {
            Global::Enabled => true,
            Global::Disabled => false,
        }
    }
}
