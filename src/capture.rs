use std::collections::HashMap;

use sexpr_ir::gast::{GAst, Handle, symbol::Symbol};




pub enum Capture {
    One(GAst),
    Many(Vec<GAst>)
}

pub type CaptureGroup = HashMap<Handle<Symbol>, Capture>;
