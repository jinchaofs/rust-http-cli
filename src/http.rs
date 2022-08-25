use anyhow::Result;
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Error, Response};
use serde::Serialize;
use std::marker::Sized;

pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    pub async fn get(&self, url: String) -> Result<Response, Error> {
        self.client.get(url).send().await
    }
    pub async fn post<T: Serialize + ?Sized>(
        &self,
        url: String,
        body: &T,
    ) -> Result<Response, Error> {
        self.client.post(url).json(&body).send().await
    }

    pub async fn print_resp(res: Response) -> Result<()> {
        print_status(&res);
        print_headers(&res);
        let mime = get_content_type(&res);
        let body = res.text().await?;
        print_body(mime, &body);
        Ok(())
    }
}

pub fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

fn print_body(mime: Option<Mime>, body: &String) {
    match mime {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}
