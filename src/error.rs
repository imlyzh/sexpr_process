use sexpr_ir::gast::GAst;


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
