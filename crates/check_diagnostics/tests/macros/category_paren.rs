use check_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(message(description = Ident))]
struct TestDiagnostic {}

fn main() {}
