use check_diagnostics::Diagnostic;

#[derive(Debug, Diagnostic)]
#[diagnostic(unknown_attr)]
struct TestDiagnostic {}

fn main() {}
