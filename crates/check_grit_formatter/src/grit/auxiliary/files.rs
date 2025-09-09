use crate::prelude::*;
use check_formatter::write;
use check_grit_syntax::{GritFiles, GritFilesFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritFiles;
impl FormatNodeRule<GritFiles> for FormatGritFiles {
    fn fmt_fields(&self, node: &GritFiles, f: &mut GritFormatter) -> FormatResult<()> {
        let GritFilesFields {
            multifile_token,
            l_curly_token,
            files,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                multifile_token.format(),
                space(),
                l_curly_token.format(),
                group(&soft_space_or_block_indent(&files.format())),
                r_curly_token.format()
            ]
        )
    }
}
