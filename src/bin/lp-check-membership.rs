use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "lp-check-membership",
    about = "Check if a user is a member of a team"
)]
struct Args {
    /// Person's user name
    person: String,

    /// Team name
    team: String,

    /// Launchpad instance to use (defaults to production)
    #[clap(short, long)]
    instance: Option<String>,
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

    let people = root.people().unwrap();

    let user = match people.get_by_name(&client, &args.person).unwrap() {
        launchpadlib::blocking::v1_0::PersonOrTeam::Person(user) => user.get(&client).unwrap(),
        _ => {
            println!("{} is not a valid user", args.person);
            std::process::exit(1);
        }
    };

    for team in user.super_teams(&client).unwrap() {
        let team = team.unwrap();
        if team.name == args.team {
            println!("{} is a member of {}", args.person, args.team);
            return;
        }
    }

    println!("{} is not a member of {}", args.person, args.team);
    std::process::exit(1);
}
