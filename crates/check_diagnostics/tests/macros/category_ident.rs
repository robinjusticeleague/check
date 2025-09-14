use check_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(category = Identifier)]
struct TestDiagnostic {}

fn main() {}
