use crate::{YamlRoot, YamlSyntaxKind};
use check_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct YamlLanguage;

impl Language for YamlLanguage {
    type Kind = YamlSyntaxKind;
    type Root = YamlRoot;
}

pub type YamlSyntaxNode = check_rowan::SyntaxNode<YamlLanguage>;
pub type YamlSyntaxToken = check_rowan::SyntaxToken<YamlLanguage>;
pub type YamlSyntaxElement = check_rowan::SyntaxElement<YamlLanguage>;
pub type YamlSyntaxNodeChildren = check_rowan::SyntaxNodeChildren<YamlLanguage>;
pub type YamlSyntaxElementChildren = check_rowan::SyntaxElementChildren<YamlLanguage>;
pub type YamlSyntaxList = check_rowan::SyntaxList<YamlLanguage>;
