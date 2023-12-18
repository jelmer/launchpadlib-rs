use clap::Parser;

#[derive(Parser)]
#[clap(name = "lp-list-bugs")]
struct Args {
    bug: Vec<u32>,

    /// Launchpad instance to use (defaults to production)
    #[clap(short, long)]
    instance: Option<String>,
}

pub const CONSUMER_KEY: &str = "lp-list-bugs";

fn main() {
    let args = Args::parse();

    let access_token = launchpadlib::auth::get_access_token(args.instance.as_deref(), CONSUMER_KEY)
        .expect("Failed to get access token");

    let client =
        launchpadlib::Client::authenticated(CONSUMER_KEY, None, &access_token.0, &access_token.1)
            .unwrap();

    let root = if let Some(host) = args.instance {
        let host = format!("api.{}", host);
        launchpadlib::v1_0::service_root_for_host(&client, &host)
    } else {
        launchpadlib::v1_0::service_root(&client)
    }
    .unwrap();

    let bugs = root.bugs().unwrap();

    for bugno in args.bug {
        let bug = bugs.get_by_id(&client, bugno).unwrap();

        println!("Bug {}: {}", bug.id, bug.title);
        for task in bug.bug_tasks(&client).unwrap() {
            let task = task.unwrap();
            println!("  {}: {}", task.bug_target_name, task.status);
        }
    }
}
