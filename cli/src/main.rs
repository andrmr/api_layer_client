use api_layer_client::{ApiResult, ApiLayerClient};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about="API Layer currency endpoints client", long_about=None)]
#[command(author, version, propagate_version=true)]
struct Cli {
    #[arg(short, long, value_name="KEY")]
    #[arg(help="API key provided by apilayer.com. Alternatively, use the APIKEY env var")]
    key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get all available currencies")]
    List,

    #[command(about = "Get the most recent exchange rate data")]
    Live {
        #[arg(short, long, value_name="SOURCE")]
        #[arg(help="Reference currency")]
        source: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let api_key = cli.key.unwrap_or(
        std::env::var("APIKEY")
            .expect("Provide an APIKEY value or set an APIKEY env var")
    );

    let client = ApiLayerClient::new(&api_key);

    match &cli.command {
        Commands::List => handle_result(client.list().await),
        Commands::Live { source } => handle_result(client.live(&source).await),
    };
}

fn handle_result<T>(result: ApiResult<T>)
where T: std::fmt::Debug
{
    println!("{:#?}", result
        .expect("Something went wrong. Make sure the APIKEY is valid"));
}