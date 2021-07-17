use sexpr_ir::gast::{GAst, constant::Constant};


#[derive(Debug, Clone)]
pub struct BadSyntax(pub GAst);


#[derive(Debug, Clone)]
pub enum CaptureError {
    BadSyntax(String),
    IsNotList(GAst),
    NotMatchConst(GAst),
    IncompleteExpr(String),
    InvalidPairRight(String),
    InvalidExprLength(usize, usize, String),
}

pub(crate) fn bad_syntax<T: ToString>(i: &T) -> CaptureError {
    CaptureError::BadSyntax(i.to_string())
}

pub(crate) fn invalid_list_tail<T: ToString>(i: &T) -> CaptureError {
    CaptureError::InvalidPairRight(i.to_string())
}

pub(crate) fn incomplete_expr<T: ToString>(i: &T) -> CaptureError {
    CaptureError::IncompleteExpr(i.to_string())
}

pub(crate) fn invalid_expr_length<T: ToString>(i: &T, takes: usize, give: usize) -> CaptureError {
    CaptureError::InvalidExprLength(takes, give, i.to_string())
}
