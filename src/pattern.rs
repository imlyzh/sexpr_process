

use sexpr_ir::gast::{GAst, Handle, constant::Constant, symbol::Symbol};

use crate::error::BadSyntax;


#[derive(Debug, Clone, PartialEq)]
pub struct ListPattern {
    pub capture_list: Vec<Pattern>,
    pub over_ignore: Option<Handle<Symbol>>,
    pub pair_right: Option<Box<Pattern>>
}


impl ListPattern {
    pub fn from(ast: &GAst) -> Result<Self, BadSyntax> {
        match ast {
            GAst::Const(_) => Err(BadSyntax(ast.clone())),
            GAst::List(lst) => {
                let pair_right = if let Some(x) = &lst.1 {
                    Some(Box::new(Pattern::from(x)?))
                } else {
                    None
                };
                if lst.0.len() == 1 {
                    let r = lst.0.get(0).unwrap();
                    let r = r
                    .get_const().ok_or_else(|| BadSyntax(ast.clone()))?
                    .get_sym().ok_or_else(|| BadSyntax(ast.clone()))?;
                    if *r.0 == "..." {
                        return Err(BadSyntax(ast.clone()));
                    }
                }
                let capture_list: Result<Vec<_>, _> = lst.0
                .iter()
                .map(|ast| Pattern::from(ast))
                .collect();
                let mut capture_list = capture_list?;
                let mut over_ignore: Option<Handle<Symbol>> = None;
                if let Pattern::Capture(end) = capture_list.last().unwrap() {
                    if *end.0 == "..." {
                        if let Pattern::Capture(name) = capture_list.clone()
                        .get(capture_list.len()-2).unwrap() {
                            capture_list = capture_list[..capture_list.len()-2].to_vec();
                            over_ignore = Some(name.clone());
                        } else {
                            return Err(BadSyntax(ast.clone()));
                        }
                    }
                }
                Ok(ListPattern { capture_list, over_ignore, pair_right })
            },
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Ignore,
    Capture(Handle<Symbol>),
    Const(Constant),
}


impl Pattern {
    pub fn from(ast: &GAst) -> Result<Self, BadSyntax> {
        let r = match ast {
            GAst::Const(c) => match c {
                Constant::Sym(name) => if *name.0 == "_" {
                    Pattern::Ignore
                } else {
                    Pattern::Capture(name.clone())
                },
                _ => Pattern::Const(c.clone())
            },
            GAst::List(s) => {
                if s.0.len() != 2 {
                    return Err(BadSyntax(ast.clone()));
                }
                if s.1.is_some() {
                    return Err(BadSyntax(ast.clone()));
                }
                let f = s.0.first().unwrap();
                let f = f
                .get_const().ok_or_else(|| BadSyntax(ast.clone()))?
                .get_sym().ok_or_else(|| BadSyntax(ast.clone()))?;
                if *f.0 != "quote" {
                    return Err(BadSyntax(ast.clone()));
                }
                let name = s.0.last().unwrap();
                let name = name
                .get_const().ok_or_else(|| BadSyntax(ast.clone()))?
                .get_sym().ok_or_else(|| BadSyntax(ast.clone()))?;
                Pattern::Const(Constant::Sym(name))
            },
        };
        Ok(r)
    }
}

