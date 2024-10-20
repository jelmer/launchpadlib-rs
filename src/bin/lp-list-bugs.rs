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

    for bugno in args.bug {
        let bug = bugs.get_by_id(&client, bugno).unwrap();

        println!("Bug {}: {}", bug.id, bug.title);
        for task in bug.bug_tasks(&client).unwrap() {
            let task = task.unwrap();
            println!("  {}: {}", task.bug_target_name, task.status);
        }
    }
}
