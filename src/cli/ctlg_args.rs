/// ctlg
#[command(
    about("catalog utility for gsr picture viewer"),
    author("ToF"),
    version,
    infer_long_args = true,
    infer_subcommands = true,
    help_template(
        "\
{before-help}{name} {version} {about} by {author-with-newline}
{usage-heading} {usage}
{all-args}{after-help}
"
    )
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// display only pictures in categorie <CATEGORIES> (e.g "foo bar")
