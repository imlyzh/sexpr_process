use std::{collections::HashMap, convert::identity};

use sexpr_ir::gast::{GAst, Handle, symbol::Symbol};

use crate::{error::CaptureError, pattern::{ListPattern, Pattern}};


pub enum Capture {
    One(GAst),
    Many(Vec<GAst>)
}

pub type CaptureBind = (Handle<Symbol>, Capture);

pub type CaptureBindList = Vec<CaptureBind>;

pub type CaptureGroup = HashMap<Handle<Symbol>, Capture>;

pub type CaptureResult = Result<CaptureBindList, CaptureError>;

pub trait Catch {
    fn catch(&self, input: &GAst) -> CaptureResult;
}


impl Catch for ListPattern {
    fn catch(&self, input: &GAst) -> CaptureResult {
        let input = input
            .get_list()
            .ok_or_else(||CaptureError::IsNotList(input.clone()))?;
        // let right_match_list: HashMap<_, _> = right_match_list.into_iter().collect();
        if input.0.len() < self.capture_list.len() {
            return Err(CaptureError::IncompleteExpr(input.to_string()));
        }
        let input_cap_list = &input.0[..self.capture_list.len()];
        let cap_list: Result<Vec<_>, _> = self.capture_list
        .iter()
        .zip(input_cap_list.iter())
        .map(|(p, a)| p.catch(a))
        .collect();
        let mut cap_list: Vec<_> = cap_list?
        .into_iter()
        .flatten()
        .collect();

        if let Some(many) = &self.over_ignore {
            cap_list.push((
                many.clone(),
                Capture::Many(input.0[self.capture_list.len()..].to_vec())));
        }
        let right_match_list = match (&self.pair_right, &input.1) {
            (None, None) => vec![],
            (Some(pattern), Some(ast)) => pattern.catch(ast)?,
            _ => return Err(CaptureError::InvalidPairRight(input.to_string())),
        };
        cap_list.extend(right_match_list);
        Ok(cap_list)
    }
}


impl Catch for Pattern {
    fn catch(&self, input: &GAst) -> CaptureResult {
        let r = match self {
            Pattern::Ignore => vec![],
            Pattern::Capture(name) => vec![(name.clone(), Capture::One(input.clone()))],
            Pattern::Const(c) => {
                let unify = input
                .get_const()
                .map(|x| &x==c)
                .map_or(false, identity);
                if !unify {
                    return Err(CaptureError::NotMatchConst(input.clone()));
                }
                vec![]
            },
        };
        Ok(r)
    }
}