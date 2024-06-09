#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisableDefaultPrefix(pub bool);

impl DisableDefaultPrefix {
    pub fn value(&self) -> bool {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Global(pub bool);

impl Global {
    pub fn value(&self) -> bool {
        self.0
    }
}
