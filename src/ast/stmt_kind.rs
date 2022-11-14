use super::{BlockDecl, LetDecl};
#[derive(Debug, PartialEq)]
pub enum StmtKind {
	LetDecl(Box<LetDecl>),
	BlockDecl(Box<BlockDecl>),
}
