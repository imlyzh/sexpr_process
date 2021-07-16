use sexpr_ir::gast::GAst;




#[derive(Debug, Clone)]
pub struct BadSyntax(pub GAst);

#[derive(Debug, Clone)]
pub struct CaptureError();

// type CaptureResult = Result<CaptureGroup, CaptureError>;
