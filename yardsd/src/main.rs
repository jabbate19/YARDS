use libyards::models::{DNSRecord, DNSZone};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
struct DNSOut {
    pub data: Vec<DNSData>,
    pub mx: Value,
    pub srv: Value,
}

#[derive(Serialize, Deserialize)]
struct DNSData {
    pub root: Option<DNSZone>,
    pub records: Option<Vec<DNSRecord>>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[tokio::main]
async fn main() {
    loop {
        println!("Checking import in named");
        // File hosts.txt must exist in the current path
        let mut contains = false;
        match read_lines("/etc/named.conf") {
            Ok(lines) => {
                // Consumes the iterator, returns an (Optional) String
                for line in lines.flatten() {
                    if line == "include \"/etc/named/yards.conf\";" {
                        contains = true;
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error Checking named.conf. Is BIND installed? Were permissions set?");
                println!("Error: {}", e);
                ::std::process::exit(1);
            }
        }
        if !contains {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("/etc/named.conf")
                .unwrap();
            match writeln!(file, "include \"/etc/named/yards.conf\";") {
                Ok(_) => {}
                Err(e) => {
                    println!("Error appending to /etc/named.conf");
                    println!("Error: {}", e);
                    ::std::process::exit(1);
                }
            }
        }
        println!("Grabbing Config...");
        let dns_data: DNSOut = reqwest::get("http://localhost:8080/api/agent/0/dns")
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let tera = Tera::new("lib/templates/*").unwrap();
        for data in &dns_data.data {
            let root = data.root.clone().unwrap().dnsroot;
            let mut file = match OpenOptions::new()
                .write(true)
                .append(true)
                .open(&format!("/var/named/{}.db", root))
            {
                Ok(file) => file,
                Err(e) => {
                    println!("Error creating db file for {}", root);
                    println!("Error: {}", e);
                    ::std::process::exit(1);
                }
            };
            let mut context = Context::new();
            context.insert("data", &data);
            context.insert("mx", &dns_data.mx);
            context.insert("srv", &dns_data.srv);
            match writeln!(file, "{}", tera.render("dnszone.db", &context).unwrap()) {
                Ok(file) => file,
                Err(e) => {
                    println!("Error writing to db file {}", root);
                    println!("Error: {}", e);
                    ::std::process::exit(1);
                }
            }
        }
        let mut context = Context::new();
        context.insert("dns_data", &dns_data.data);
        println!("{}", tera.render("yards.conf", &context).unwrap());
    }
}
