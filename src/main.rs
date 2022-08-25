use anyhow::{anyhow, Ok, Result};
use clap::{AppSettings, Args, Parser, Subcommand};
use reqwest::Url;
use std::{collections::HashMap, str::FromStr};

mod http;
use http::Http;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[clap(setting = AppSettings::ColoredHelp)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get Request Command
    Get(Get),
    /// Post Request Command
    Post(Post),
}

#[derive(Args, Debug)]
/// Get Request Struct
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    /// Request Url
    url: String,
}
#[derive(Args, Debug)]
/// Post Request Struct
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    /// Request Url
    url: String,
    #[clap(parse(try_from_str = parse_body_kv_pair))]
    /// Request body
    body: Vec<PostBodyKVPair>,
}
fn parse_url(url: &str) -> Result<String> {
    let _: Url = url.parse()?;
    Ok(url.into())
}

fn parse_body_kv_pair(s: &str) -> Result<PostBodyKVPair> {
    Ok(s.parse()?)
}
/// 请求体键值对结构体
#[derive(Debug)]
struct PostBodyKVPair {
    key: String,
    value: String,
}

impl FromStr for PostBodyKVPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s1 = s.replace(" ", "");
        let mut split = s1.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            key: split.next().ok_or_else(err)?.to_string(),
            value: split.next().ok_or_else(err)?.to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let http = Http::new();

    let _result = match cli.command {
        Commands::Get(ref args) => {
            println!("{:?}", &args);
            http.get(args.url.clone()).await?
        }
        Commands::Post(ref args) => {
            println!("{:?}", &args);
            let mut body = HashMap::new();
            for pair in args.body.iter() {
                body.insert(&pair.key, &pair.value);
            }
            http.post(args.url.clone(), &body).await?
        }
    };
    Http::print_resp(_result).await?;
    Ok(())
}
