use clap::{App, Arg, SubCommand};
use dao_framework::dao::{Dao, Member, Proposal};

type KankunDao = Arc<Mutex<Dao>>;

#[kankun::main]
async fn main() {
    let matches = App::new("Rustocrat")
        .version("0.1.0")
        .author("Rafael Fuentes Rangel")
        .about("DAO")
        .subcommand(
            SubCommand::with_name("create_member")
                .about("Create a new member")
                .arg(Arg::with_name("address").required(true).help("Member address"))
                .arg(Arg::with_name("voting_power").required(true).help("Member voting power")),
        )
        .subcommand(
            SubCommand::with_name("create_proposal")
                .about("Create a new proposal")
                .arg(Arg::with_name("id").required(true).help("Proposal ID"))
                .arg(Arg::with_name("title").required(true).help("Proposal title"))
                .arg(Arg::with_name("description").required(true).help("Proposal description"))
                .arg(Arg::with_name("amount").required(true).help("Proposal amount"))
                .arg(Arg::with_name("beneficiary").required(true).help("Proposal beneficiary")),
        )
        .get_matched();

    let dao = Arc::new(Mutex::new(Dao::new()));

    let create_member_route = wrap::path!("members");
        .and(warp::post())
        .and(warp::body::json())
}

