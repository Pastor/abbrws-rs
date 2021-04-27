use structopt::clap::AppSettings;
use structopt::StructOpt;
use yansi::Paint;

use abbrws::Signal;

#[derive(StructOpt)]
#[structopt(setting(AppSettings::DeriveDisplayOrder))]
#[structopt(setting(AppSettings::ColoredHelp))]
#[structopt(setting(AppSettings::UnifiedHelpMessage))]
struct Options {
    /// The host to connect to.
    #[structopt(long, short)]
    #[structopt(default_value = "127.0.0.1")]
    host: String,

    /// The user to authenticate as.
    #[structopt(long, short)]
    #[structopt(global = true)]
    #[structopt(default_value = "Default User")]
    user: String,

    /// The password for the user.
    #[structopt(long, short)]
    #[structopt(global = true)]
    #[structopt(default_value = "robotics")]
    password: String,

    #[structopt(long)]
    #[structopt(group = "coordinate")]
    #[structopt(default_value = "0")]
    coord_x: i32,

    #[structopt(long)]
    #[structopt(group = "coordinate")]
    #[structopt(default_value = "0")]
    coord_y: i32,

    #[structopt(long)]
    #[structopt(group = "coordinate")]
    #[structopt(default_value = "0")]
    coord_z: i32,
}

#[tokio::main]
async fn main() {
    if let Err(e) = do_main(&Options::from_args()).await {
        eprintln!("{} {}", Paint::red("Error:").bold(), e);
        std::process::exit(1);
    }
}

/// https://developercenter.robotstudio.com/api/rwsApi/
/// https://developercenter.robotstudio.com/api/rwsApi/msh_actions_page.html
async fn do_main(options: &Options) -> Result<(), String> {
    let connect = || {
        abbrws::Client::new(&options.host, &options.user, &options.password)
            .map_err(|e| format!("failed to connect to {:?}: {}", options.host, e))
    };

    eprintln!("user: {}", options.user);

    let mut client = connect()?;

    // parse_result_signals("iosystem_signals", abbrws::Client::get_signals(&mut client).await);
    // parse_result_text("rw", abbrws::Client::rw(&mut client).await);
    parse_result_text("grant_rmmp", abbrws::Client::grant_rmmp(&mut client).await);
    // parse_result_text("mastership_domain_request", abbrws::Client::mastership_domain_request(&mut client, "motion").await);
    parse_result("mastership_request", abbrws::Client::mastership_request(&mut client).await);
    let coord = options.coord_x + options.coord_y + options.coord_z;
    eprintln!("Coordinate: {}", coord);
    parse_result("mastership_release", abbrws::Client::mastership_release(&mut client).await);
    Ok(())
}

fn parse_result(name: &'static str, result: Result<(), abbrws::Error>) {
    match result {
        Ok(_) => eprintln!("ABB[{}]: success", name),
        Err(err) => eprintln!("ABB[{}]: {:?}", name, err),
    }
}

fn parse_result_text(name: &'static str, result: Result<String, abbrws::Error>) {
    match result {
        Ok(message) => eprintln!("ABB[{}]: success. Message: {:?}", name, message),
        Err(err) => eprintln!("ABB[{}]: {:?}", name, err),
    }
}

fn parse_result_signals(name: &'static str, result: Result<Vec<Signal>, abbrws::Error>) {
    match result {
        Ok(message) => eprintln!("ABB[{}]: success. Message: {:#?}", name, message),
        Err(err) => eprintln!("ABB[{}]: {:?}", name, err),
    }
}
