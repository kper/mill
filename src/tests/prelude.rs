pub use crate::codegen::Codegen;
pub use crate::pass::Pass;
pub use crate::runner::Runner;
pub use crate::traversal::*;
pub use crate::visitors::*;

pub use crate::run_visitor;

#[macro_export]
macro_rules! run_visitor {
    ($input:expr, $passes:expr) => {{
        let _ = env_logger::builder().is_test(true).try_init();

        let input = $input;

        let mut program = crate::grammar::ProgramParser::new().parse(&input).unwrap();

        let mut runner = Runner;
        //runner.run_visitors($passes, &mut program)
    }};
}
