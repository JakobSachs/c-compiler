use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod generate;
lalrpop_mod!(pub grammar);
