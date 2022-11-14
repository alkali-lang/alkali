use super::Stmt;

#[derive(Debug, PartialEq)]
pub struct BlockDecl {
	pub stmts: Vec<Stmt>,
}
