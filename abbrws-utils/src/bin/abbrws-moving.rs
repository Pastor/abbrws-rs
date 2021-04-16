use structopt::clap::AppSettings;
use structopt::StructOpt;
use yansi::Paint;

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

async fn do_main(options: &Options) -> Result<(), String> {
    let connect = || {
        abbrws::Client::new(&options.host, &options.user, &options.password)
            .map_err(|e| format!("failed to connect to {:?}: {}", options.host, e))
    };

    eprintln!("user: {}", options.user);

    let mut client = connect()?;
    master(&mut client, |_c: &mut abbrws::Client| {
        let _coord = options.coord_x + options.coord_y + options.coord_z;
        true
    })
    .await;
    Ok(())
}

#[allow(unused_must_use)]
async fn master<Fun>(c: &mut abbrws::Client, mut f: Fun) -> bool
where
    Fun: FnMut(&mut abbrws::Client) -> bool,
{
    let mut result = false;
    abbrws::Client::mastership_request(c).await.and_then(|_| {
        Ok(async {
            result = f(c);
            abbrws::Client::mastership_release(c).await;
            ()
        })
    });
    result
}
