/// Tests the printing of AST nodes.
fn main() {
    use lifesharp::{ast, identifier::Id};

    let parameter_name = ast::Located::new(Id::new("n").unwrap(), 0, 0);

    let mut function_definition =
        ast::FunctionDefinition::new(ast::Located::new(Id::new("test").unwrap(), 0, 0));

    function_definition.body.push(ast::Located::new(
        ast::Expression::Name(parameter_name.clone()),
        0,
        0,
    ));

    let parameter_type_path = ast::PathId::global(vec![
        ast::Located::new(Id::new("core").unwrap(), 0, 0),
        ast::Located::new(Id::new("helpers").unwrap(), 0, 0),
    ]);

    let mut parameter = ast::Parameter::new(ast::Type::Named(ast::TypeId::new(
        parameter_type_path,
        ast::Located::new(Id::new("MyType").unwrap(), 0, 0),
    )));
    parameter.pattern = ast::Pattern::Name(parameter_name);

    function_definition.parameters.push(parameter);

    let mut tree = ast::Tree::default();
    tree.declarations.push(function_definition.into());

    println!("{}", tree);
}
