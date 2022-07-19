use std::io::{BufRead, Write};

fn cli(runtime: &actix_web::rt::Runtime, port: u16) -> anyhow::Result<()> {
    runtime.block_on(async {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let client = reqwest::Client::new();
        let url = format!("http://localhost:{}/greet", port);

        let mut line = String::new();
        loop {
            line.clear();
            stdout.write_all(b"Enter your name:")?;
            stdout.flush()?;
            line.clear();
            let length = stdin.read_line(&mut line)?;

            if length == 0 {
                return Ok(());
            }

            let name = line.trim();
            if !name.is_empty() {
                let resp = bson_http::client::make_request(&client, line.trim(), &url).await?;
                write!(stdout, "{}    [{}]\n", resp.message, resp.time)?;
            }
        }
    })
}

fn main() -> anyhow::Result<()> {
    let port: u16 = 32844;
    let runtime = actix_web::rt::Runtime::new()?;

    runtime.spawn(bson_http::server::server(port));
    cli(&runtime, port)?;
    Ok(())
}
