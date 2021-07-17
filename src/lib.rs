pub mod pattern;
pub mod capture;
pub mod error;


#[cfg(test)]
mod tests {
    use sexpr_ir::syntax::sexpr;

    use crate::pattern::{ListPattern, Pattern};

    #[test]
    fn it_works() {
        let sexpr = sexpr::one_unit_parse(
            "name",
             "<sexpr-process>").unwrap();
        if let Pattern::Capture(_) = Pattern::from(&sexpr).unwrap() {
            println!("ok");
        } else {
            unreachable!()
        }
        let sexpr = sexpr::one_unit_parse(
            "114514",
             "<sexpr-process>").unwrap();
        if let Pattern::Const(_) = Pattern::from(&sexpr).unwrap() {
            println!("ok");
        } else {
            unreachable!()
        }
        let sexpr = sexpr::one_unit_parse(
            "'name",
             "<sexpr-process>").unwrap();
        if let Pattern::Const(_) = Pattern::from(&sexpr).unwrap() {
            println!("ok");
        } else {
            unreachable!()
        }
        let sexpr = sexpr::one_unit_parse(
            "_",
             "<sexpr-process>").unwrap();
        if let Pattern::Ignore = Pattern::from(&sexpr).unwrap() {
            println!("ok");
        } else {
            unreachable!()
        }
        let sexpr = sexpr::one_unit_parse(
            "('assign name expr ...)",
             "<sexpr-process>").unwrap();
        let _pattern = ListPattern::from(&sexpr).unwrap();

    }
}
