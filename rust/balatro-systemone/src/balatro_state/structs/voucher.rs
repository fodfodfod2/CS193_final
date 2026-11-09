use super::*;

/* This will represent a Voucher */
pub struct Voucher {
    name: String,
    desc: String,
}

impl Voucher {
    pub fn new(name: String, desc: String) -> Self {
        Voucher { name, desc }
    }
}
