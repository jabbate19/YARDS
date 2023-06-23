use chrono::serde::ts_seconds::serialize as to_ts;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    types::chrono::{DateTime, Utc},
    FromRow, Pool, Postgres,
};
use utoipa::ToSchema;

pub struct AppState {
    pub db: Pool<Postgres>,
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
pub struct Device {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub comments: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Interface {
    pub id: i32,
    pub macaddr: String,
    pub deviceid: i32,
    pub name: String,
    pub comments: String,
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

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct StaticAddress {
    pub addressid: i32,
    pub ipaddr: String,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, ToSchema, Debug, PartialEq, Copy)]
#[sqlx(type_name = "ipversion")]
pub enum IPVersion {
    V4,
    V6,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct IPRange {
    pub id: i32,
    pub name: String,
    pub ipversion: IPVersion,
    pub networkid: String,
    pub cidr: i32,
    pub description: String,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Server {
    pub id: i32,
    pub name: String,
    pub tokenhash: Option<String>,
    //#[serde(serialize_with = "to_ts")]
    pub lastcheckin: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DDNS {
    pub iprangeid: i32,
    pub zoneid: i32,
}

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DNSZone {
    pub id: i32,
    pub zonename: String,
    pub dnsroot: String,
    pub serverid: i32,
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

#[derive(Debug, sqlx::Type, Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct DHCPRange {
    pub id: i32,
    pub iprangeid: i32,
    pub name: String,
    pub dhcpstart: String,
    pub dhcpend: String,
    pub gateway: String,
    pub default_dns: String,
    pub lease_time: i32,
    pub serverid: i32,
}
