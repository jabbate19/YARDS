use anyhow::{anyhow, Result};
use libyards::models::{DNSRecord, DNSZone};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::OpenOptions, io::Write};
use tera::{Context, Tera};

#[derive(Serialize, Deserialize)]
struct DNSOut {
    pub data: Vec<DNSData>,
    pub mx: Value,
    pub srv: Value,
}

#[derive(Serialize, Deserialize)]
struct RecordWithSpacing {
    record: DNSRecord,
    key_ttl_spacing: String,
    ttl_in_spacing: String,
    recordtype_value_spacing: String,
}

#[derive(Serialize, Deserialize)]
struct DNSData {
    pub root: Option<DNSZone>,
    pub records: Option<Vec<DNSRecord>>,
}

#[derive(Serialize, Deserialize)]
struct DNSDataWithSpacing {
    pub root: Option<DNSZone>,
    pub records: Option<Vec<RecordWithSpacing>>,
}

impl From<DNSData> for DNSDataWithSpacing {
    fn from(value: DNSData) -> DNSDataWithSpacing {
        let mut records_with_spacing: Vec<RecordWithSpacing> = Vec::new();
        for record in value.records.unwrap() {
            let key_ttl_spacing = " ".repeat(16 - record.key.len());
            let ttl_in_spacing = " ".repeat((16 - record.ttl.ilog10()).try_into().unwrap());
            let recordtype_value_spacing = " ".repeat(16 - record.recordtype.len());
            records_with_spacing.push(RecordWithSpacing {
                record,
                key_ttl_spacing,
                ttl_in_spacing,
                recordtype_value_spacing,
            });
        }
        DNSDataWithSpacing {
            root: value.root,
            records: Some(records_with_spacing),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct DNSOutWithSpacing {
    pub data: Vec<DNSDataWithSpacing>,
    pub mx: Value,
    pub srv: Value,
}

impl From<DNSOut> for DNSOutWithSpacing {
    fn from(value: DNSOut) -> DNSOutWithSpacing {
        let mut data_with_spacing: Vec<DNSDataWithSpacing> = Vec::new();
        for data in value.data {
            data_with_spacing.push(DNSDataWithSpacing::from(data));
        }
        DNSOutWithSpacing {
            data: data_with_spacing,
            mx: value.mx,
            srv: value.srv,
        }
    }
}

pub async fn main(host: &str) -> Result<()> {
    println!("Grabbing Config...");
    let dns_data: DNSOut = reqwest::get(format!("{}/api/agent/0/dns", host))
        .await?
        .json()
        .await?;
    let tera = Tera::new("lib/templates/*")?;
    let dns_data = DNSOutWithSpacing::from(dns_data);
    for data in &dns_data.data {
        let root = data.root.clone().ok_or(anyhow!("Root not found"))?.dnsroot;
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&format!("{}.db", root))
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
        match writeln!(file, "{}", tera.render("dnszone.db", &context)?) {
            Ok(file) => file,
            Err(e) => {
                println!("Error writing to db file {}", root);
                println!("Error: {}", e);
                ::std::process::exit(1);
            }
        }
    }
    Ok(())
}
