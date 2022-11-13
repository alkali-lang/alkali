use super::LetDecl;
#[derive(Debug)]
pub enum StmtKind {
	LetDecl(Box<LetDecl>),
}
