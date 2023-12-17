use clap::Parser;

#[derive(Parser)]
#[clap(name = "lp-set-dupe")]
struct Args {
    main_bug: u32,

    dupes: Vec<u32>,

    /// Launchpad instance to use (defaults to production)
    #[clap(short, long)]
    instance: Option<String>,

    /// Skip confirmation prompt
    #[clap(short, long)]
    force: bool,
}

pub const CONSUMER_KEY: &str = "lp-set-dupe";

fn main() {
    let args = Args::parse();

    let access_token = launchpadlib::auth::cmdline_access_token(args.instance.as_deref(), CONSUMER_KEY).unwrap();

    let client = launchpadlib::Client::authenticated(CONSUMER_KEY, None, &access_token.0, &access_token.1).unwrap();

    let root = if let Some(host) = args.instance {
        let host = format!("api.{}", host);
        launchpadlib::v1_0::service_root_for_host(&client, &host)
    } else {
        launchpadlib::v1_0::service_root(&client)
    }.unwrap();

    let bugs = root.bugs().unwrap();

    let mut main_bug = bugs.get_by_id(&client, args.main_bug).unwrap();

    log::info!("Marking bugs as duplicate of bug {} ({})", main_bug.id, main_bug.title);

    if let Some(new_main_dupe_of) = main_bug.duplicate_of().as_ref().map(|c| c.get(&client).unwrap()) {
        log::error!("Bug {} is already a duplicate of bug {} ({})", main_bug.id, new_main_dupe_of.id, new_main_dupe_of.title);
        println!("Would you like to use {} as the new main bug instead? [y/N]", new_main_dupe_of.id);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "y" | "Y" => {
                main_bug.self_().unwrap().patch(&client, &launchpadlib::v1_0::BugDiff {
                    duplicate_of_link: new_main_dupe_of.self_link.clone(),
                    ..Default::default()
                }).unwrap();
                println!("Bug {} is now a duplicate of bug {} ({})", main_bug.id, new_main_dupe_of.id, new_main_dupe_of.title);
                main_bug = new_main_dupe_of;
            },
            _ => {
                log::error!("Aborting");
                std::process::exit(1);
            }
        }
    }

    let mut bugs_to_process = args.dupes.clone();

    while let Some(bugno) = bugs_to_process.pop() {
        if let Ok(bug) = bugs.get_by_id(&client, bugno) {
            println!("Marking bug {} as duplicate of bug {} ({})... ", bugno, bug.id, bug.title);
            if bug.duplicate_of().is_some() {
                log::error!("Bug {} is already a duplicate of another bug", bug.id);
                std::process::exit(1);
            }
            bug.self_().unwrap().patch(&client, &launchpadlib::v1_0::BugDiff {
                duplicate_of_link: main_bug.self_link.clone(),
                ..Default::default()
            }).unwrap();
            println!("done");
            bugs_to_process.extend(bug.duplicates(&client).unwrap().map(|bug| bug.unwrap().id as u32));
        } else {
            log::error!("Bug {} not found", bugno);
            std::process::exit(1);
        }
    }
}
