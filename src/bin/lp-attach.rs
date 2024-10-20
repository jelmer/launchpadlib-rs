use clap::Parser;

#[derive(Parser)]
#[clap(name = "lp-attach")]
struct Args {
    number: u32,

    path: Option<std::path::PathBuf>,

    /// Launchpad instance to use (defaults to production)
    #[clap(short, long)]
    instance: Option<String>,

    /// Comment to add to the attachment
    #[clap(long, default_value = "")]
    comment: String,

    /// Filename to use for the attachment
    #[clap(long)]
    filename: Option<String>,

    /// URL to use for the attachment
    #[clap(long)]
    url: Option<url::Url>,

    /// Whether the attachment is a patch
    #[clap(long)]
    is_patch: Option<bool>,

    /// Content type of the attachment
    #[clap(long)]
    content_type: Option<String>,

    /// Description of the attachment
    #[clap(long)]
    description: Option<String>,
}

pub const CONSUMER_KEY: &str = "lp-attach";

fn main() {
    let args = Args::parse();

    let client =
        launchpadlib::blocking::Client::authenticated(args.instance.as_deref(), CONSUMER_KEY)
            .unwrap();

    let root = if let Some(host) = args.instance {
        let host = format!("api.{}", host);
        launchpadlib::blocking::v1_0::service_root_for_host(&client, &host)
    } else {
        launchpadlib::blocking::v1_0::service_root(&client)
    }
    .unwrap();

    let bugs = root.bugs().unwrap();

    let bug = bugs.get_by_id(&client, args.number).unwrap();

    log::info!("Adding attachment to bug {} ({})", bug.id, bug.title);

    let bug = bug.self_().unwrap();

    let part = if let Some(path) = args.path.as_ref() {
        reqwest::blocking::multipart::Part::file(path).unwrap()
    } else {
        let io = std::io::stdin();
        reqwest::blocking::multipart::Part::reader(io)
            .file_name("attachment")
            .mime_str("application/octet-stream")
            .unwrap()
    };

    bug.add_attachment(
        &client,
        Some(part),
        &args.comment,
        args.filename.as_deref(),
        args.url.as_ref(),
        args.is_patch.as_ref(),
        args.content_type.as_deref(),
        args.description.as_deref(),
    )
    .unwrap();
}
