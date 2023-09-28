use actix_web::{
    get, post,
    web::{Data, Path},
    HttpResponse, Responder,
};
use ipnetwork::Ipv4Network;
use libyards::models::{
    Address, AppState, DHCPOut, DHCPRange, DNSRecord, DNSZone, IPOut, IPRange, IPRangeOut,
    MXRecord, SRVRecord, VLANData,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};

#[derive(Serialize, Deserialize)]
struct DNSData {
    pub root: Option<DNSZone>,
    pub records: Option<Vec<DNSRecord>>,
}

#[derive(Serialize, Deserialize)]
struct Roles {
    dns: bool,
    dhcp: bool,
}

#[utoipa::path(
    context_path = "/api/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{serverid}/roles")]
async fn get_server_roles(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    match query_as!(
        Roles,
        "SELECT EXISTS(SELECT * FROM dnszone WHERE serverid = $1) AS \"dns!\", EXISTS(SELECT * FROM vlan WHERE serverid = $1) AS \"dhcp!\"",
        serverid
    ).fetch_one(&state.db).await {
        Ok(roles) => HttpResponse::Ok().json(roles),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    context_path = "/api/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{serverid}/dns")]
async fn generate_dns_data(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    let update = query!("SELECT dnsupdate FROM server WHERE id = $1", serverid)
        .fetch_one(&state.db)
        .await
        .unwrap();
    if !update.dnsupdate {
        return HttpResponse::NotModified().finish();
    }
    let core_data = match query_as!(
        DNSData,
        "SELECT ROW(dnszone.*)::dnszone AS \"root: DNSZone\", array_remove(ARRAY_AGG(dnsrecord.*), NULL) AS \"records: Vec<DNSRecord>\" FROM dnszone LEFT JOIN dnsrecord ON dnsrecord.zoneid = dnszone.id WHERE dnszone.serverid = $1 GROUP BY dnszone.id",
        serverid
    ).fetch_all(&state.db).await {
        Ok(zones) => zones,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let mx_data = match query_as!(
        MXRecord,
        "SELECT * FROM mxrecord WHERE id IN (SELECT id FROM dnsrecord WHERE zoneid IN (SELECT id FROM dnszone WHERE serverid = $1))",
        serverid
    ).fetch_all(&state.db).await {
        Ok(records) => records,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let srv_data = match query_as!(
        SRVRecord,
        "SELECT * FROM srvrecord WHERE id IN (SELECT id FROM dnsrecord WHERE zoneid IN (SELECT id FROM dnszone WHERE serverid = $1))",
        serverid
    ).fetch_all(&state.db).await {
        Ok(records) => records,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };
    let mut out = json!({
        "data": core_data,
        "mx": {},
        "srv": {}
    });
    for mx in mx_data {
        out["mx"]
            .as_object_mut()
            .unwrap()
            .insert(mx.id.to_string(), serde_json::to_value(mx).unwrap());
    }
    for srv in srv_data {
        out["srv"]
            .as_object_mut()
            .unwrap()
            .insert(srv.id.to_string(), serde_json::to_value(srv).unwrap());
    }
    HttpResponse::Ok().json(out)
}

#[utoipa::path(
    context_path = "/api/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[get("/{serverid}/dhcp")]
async fn generate_dhcp_data(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    let update = query!("SELECT dhcpupdate FROM server WHERE id = $1", serverid)
        .fetch_one(&state.db)
        .await
        .unwrap();
    if !update.dhcpupdate {
        return HttpResponse::NotModified().finish();
    }
    let mut out: Vec<DHCPOut> = Vec::new();
    let vlans: Vec<VLANData> = query_as!(
        VLANData,
        "SELECT vlan.id, vlan.name, ARRAY_AGG(iprange.*) AS \"ranges: Vec<IPRange>\" FROM vlan JOIN iprange ON iprange.vlan = vlan.id WHERE vlan.serverid = $1 GROUP BY vlan.id",
        serverid
    )
    .fetch_all(&state.db)
    .await.unwrap();
    for vlan in vlans {
        let mut dhcpout = DHCPOut {
            vlan_id: vlan.id,
            vlan_name: vlan.name,
            ranges: Vec::new(),
        };
        for range in vlan.ranges.unwrap() {
            let dhcp: Vec<DHCPRange> = query_as!(
                DHCPRange,
                "SELECT * FROM dhcprange WHERE iprangeid = $1",
                range.id
            )
            .fetch_all(&state.db)
            .await
            .unwrap();
            let mut iprangeout = IPRangeOut {
                iprange: range.clone(),
                dhcp,
                netmask: Ipv4Network::new(
                    range.networkid.parse().unwrap(),
                    range.cidr.try_into().unwrap(),
                )
                .unwrap()
                .mask()
                .to_string(),
                addresses: Vec::new(),
            };
            let addresses = query_as!(
                IPOut,
                "SELECT ROW(address.*)::address AS \"address: Address\", staticaddress.ipaddr AS \"static_addr: _\", interface.macaddr AS \"interface: _\", device.name AS \"name: _\", device.owner AS \"owner: _\", encode(digest(concat(device.name, interface.macaddr), 'sha1'), 'hex') AS \"hash\" FROM address JOIN interface ON address.interfaceid = interface.id JOIN device ON interface.deviceid = device.id LEFT JOIN staticaddress ON staticaddress.addressid = address.id WHERE address.iprangeid = $1",
                range.id
            )
            .fetch_all(&state.db)
            .await.unwrap();
            for address in addresses {
                iprangeout.addresses.push(address);
            }
            dhcpout.ranges.push(iprangeout);
        }
        out.push(dhcpout);
    }
    HttpResponse::Ok().json(out)
}

#[utoipa::path(
    context_path = "/api/agent",
    responses(
        (status = 200, description = "Provide Data for Server"),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/{serverid}/success")]
async fn success(state: Data<AppState>, path: Path<(i32,)>) -> impl Responder {
    let (serverid,) = path.into_inner();
    query!(
        "UPDATE server SET dhcpupdate = false, dnsupdate = false, lastcheckin = now() WHERE id = $1",
        serverid
    ).execute(&state.db).await.unwrap();
    HttpResponse::Ok().finish()
}
