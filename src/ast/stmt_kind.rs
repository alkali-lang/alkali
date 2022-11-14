use super::LetDecl;
#[derive(Debug, PartialEq)]
pub enum StmtKind {
	LetDecl(Box<LetDecl>),
}
