type Ast<'ast> = &'ast AstStructure<'ast>;

struct AstStructure<'ast> {
    id: NodeId,
    _: AstKind<'ast>
}

enum AstKind<'ast> {
    ExprInt,
    ExprLambda(Ast<'ast>),
}

fn compute_types<'tcx,'ast>(ast: Ast<'ast>) -> Type<'tcx>
{
    match ast.kind {}
}
