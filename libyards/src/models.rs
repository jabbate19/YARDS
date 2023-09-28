use std::{fmt::Display, collections::HashMap, sync::{Arc}};
use chrono::serde::ts_seconds::serialize as to_ts;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    types::chrono::{DateTime, Utc},
    FromRow, Pool, Postgres,
};
use utoipa::ToSchema;
use openssl::rsa::Rsa;
use openssl::pkey::{PKey, Public};
use futures::lock::Mutex;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub jwt_cache: Arc<Mutex<HashMap<String, PKey<Public>>>>
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct APIKey {
    pub id: i32,
    pub name: String,
    pub keyhash: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct KeyPermissions {
    pub id: i32,
    pub keyid: i32,
    pub permission: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Logs {
    pub id: i32,
    #[serde(serialize_with = "to_ts")]
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Vlan {
    pub id: i32,
    pub name: String,
    pub serverid: i32,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub comments: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Interface {
    pub id: i32,
    pub macaddr: String,
    pub deviceid: i32,
    pub name: String,
    pub comments: String,
}

impl PgHasArrayType for Interface {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_interface")
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq)]
#[sqlx(type_name = "iptype")]
pub enum IPType {
    Static,
    Dynamic,
}

#[derive(sqlx::Type,Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Address {
    pub id: i32,
    pub interfaceid: i32,
    pub iprangeid: i32,
    pub iptype: IPType,
}

impl PgHasArrayType for Address {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_address")
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct StaticAddress {
    pub addressid: i32,
    pub ipaddr: String,
}

impl PgHasArrayType for StaticAddress {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_staticaddress")
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Copy)]
#[sqlx(type_name = "ipversion")]
pub enum IPVersion {
    V4,
    V6,
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct IPRange {
    pub id: i32,
    pub name: String,
    pub ipversion: IPVersion,
    pub networkid: String,
    pub cidr: i32,
    pub description: String,
    pub vlan: i32,
    pub gateway: String,
    pub default_dns: String,
    pub dns_domain: String,
    pub default_lease_time: i32,
    pub max_lease_time: i32,
    pub min_lease_time: i32,
}

impl PgHasArrayType for IPRange {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_iprange")
    }
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub tokenhash: Option<String>,
    //#[serde(serialize_with = "to_ts")]
    pub lastcheckin: Option<DateTime<Utc>>,
    pub dnsupdate: bool,
    pub dhcpupdate: bool,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DDNS {
    pub iprangeid: i32,
    pub zoneid: i32,
}

#[derive(sqlx::Type,Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DNSZone {
    pub id: i32,
    pub zonename: String,
    pub serverid: i32,
    pub dnsroot: String,
    pub refresh: i32,
    pub retry: i32,
    pub expire: i32,
    pub nxdomain: i32,
    pub contact: String,
    pub server: String,
}

#[derive(sqlx::Type, Copy, Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq)]
#[sqlx(type_name = "dnsrecordtype")]
pub enum DNSRecordType {
    A,
    AAAA,
    NS,
    MX,
    CNAME,
    SOA,
    SRV,
    PTR,
    TXT,
}

impl Display for DNSRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNSRecordType::A => write!(f, "A"),
            DNSRecordType::AAAA => write!(f, "AAAA"),
            DNSRecordType::NS => write!(f, "NS"),
            DNSRecordType::MX => write!(f, "MX"),
            DNSRecordType::CNAME => write!(f, "CNAME"),
            DNSRecordType::SOA => write!(f, "SOA"),
            DNSRecordType::SRV => write!(f, "SRV"),
            DNSRecordType::PTR => write!(f, "PTR"),
            DNSRecordType::TXT => write!(f, "TXT"),
        }
    }
}

impl DNSRecordType {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DNSRecord {
    pub id: i32,
    pub zoneid: i32,
    pub key: String,
    pub ttl: i32,
    pub value: String,
    pub recordtype: DNSRecordType,
}

impl PgHasArrayType for DNSRecord {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_dnsrecord")
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct MXRecord {
    pub id: i32,
    pub preference: i32
}

#[derive(sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct SRVRecord {
    pub id: i32,
    pub preference: i32,
    pub weight: i32,
    pub port: i32
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DHCPRange {
    pub id: i32,
    pub iprangeid: i32,
    pub name: String,
    pub dhcpstart: String,
    pub dhcpend: String,
    pub jail: bool,
}

#[derive(Serialize, Deserialize)]
pub struct VLANData {
    pub id: i32,
    pub name: String,
    pub ranges: Option<Vec<IPRange>>,
}

#[derive(Serialize, Deserialize)]
pub struct IPOut {
    pub name: String,
    pub owner : String,
    pub interface: String,
    pub address: Option<Address>,
    pub hash: Option<String>,
    pub static_addr: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IPRangeOut {
    pub iprange: IPRange,
    pub netmask: String,
    pub dhcp: Vec<DHCPRange>,
    pub addresses: Vec<IPOut>,
}

#[derive(Serialize, Deserialize)]
pub struct DHCPOut {
    pub vlan_id: i32,
    pub vlan_name: String,
    pub ranges: Vec<IPRangeOut>,
}