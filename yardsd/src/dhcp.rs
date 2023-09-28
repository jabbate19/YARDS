use anyhow::Result;
use libyards::models::DHCPOut;
use std::fs::OpenOptions;
use std::io::Write;
use tera::Context;
use tera::Tera;

pub async fn main(host: &str) -> Result<()> {
    println!("Grabbing Config...");
    let dhcp_data: Vec<DHCPOut> = reqwest::get(format!("{}/api/agent/0/dhcp", host))
        .await?
        .json()
        .await?;
    let tera = Tera::new("lib/templates/*")?;
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open("yards.conf")
    {
        Ok(file) => file,
        Err(e) => {
            println!("Error creating conf file");
            println!("Error: {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("vlans", &dhcp_data);
    match writeln!(file, "{}", tera.render("dhcpd.conf", &context)?) {
        Ok(file) => file,
        Err(e) => {
            println!("Error writing to db file");
            println!("Error: {}", e);
            ::std::process::exit(1);
        }
    }
    Ok(())
}
