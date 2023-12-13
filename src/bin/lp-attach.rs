use clap::Parser;

#[derive(Parser)]
#[clap(name = "lp-attach")]
struct Args {
    number: u32,

    /// Launchpad instance to use (defaults to production)
    #[clap(short, long)]
    instance: Option<String>,
}

pub const CONSUMER_KEY: &str = "lp-attach";

fn main() {
    let access_token = launchpadlib::auth::cmdline_access_token(CONSUMER_KEY).unwrap();

    let client = launchpadlib::Client::authenticated(CONSUMER_KEY, None, &access_token.0, &access_token.1).unwrap();

    let args = Args::parse();

    let root = if let Some(host) = args.instance {
        launchpadlib::v1_0::service_root_for_host(&client, &host)
    } else {
        launchpadlib::v1_0::service_root(&client)
    }.unwrap();

    let bugs = root.bugs().unwrap();

    let bug = bugs.get_by_id(&client, args.number).unwrap();

    let bug = bug.self_().unwrap();
}
