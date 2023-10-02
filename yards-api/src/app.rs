use crate::auth::update_cache;
use crate::{admin::*, agent::*, device::*};
use actix_web::web::{self, scope, Data};
use libyards::models::*;

use futures::lock::Mutex;
use openssl::pkey::{PKey, Public};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::{collections::HashMap, env};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            iprange::get_ip_range,
            iprange::add_ip_range,
            iprange::delete_ip_range,
            iprange::edit_ip_range,
            dhcprange::get_ip_range_dhcp,
            dhcprange::add_ip_range_dhcp,
            dhcprange::delete_ip_range_dhcp,
            dhcprange::edit_ip_range_dhcp,
            server::get_servers,
            server::add_server,
            server::delete_server,
            ddns::get_ddns,
            ddns::add_ddns,
            ddns::delete_ddns,
            dnszone::get_dns_zones,
            dnszone::add_dns_zone,
            dnszone::delete_dns_zone,
            dnszone::edit_dns_zone,
            dnsrecord::get_dns_zone_records,
            dnsrecord::add_dns_zone_record,
            dnsrecord::delete_dns_zone_record,
            dnsrecord::edit_dns_zone_record
        ),
        components(
            schemas(APIKey, KeyPermissions, Logs, Device, Interface, Address, IPType, StaticAddress, IPVersion, IPRange, Server, DDNS, DNSZone, DNSRecord, DNSRecordType, DHCPRange)
        ),
        tags(
            (name = "YARDS", description = "Yet Another Registering Devices Service")
        ),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("frontend_api_key"))),
            )
        }
    }

    let openapi = ApiDoc::openapi();
    cfg.service(
        scope("/api")
            .service(
                scope("/admin")
                    .service(iprange::get_ip_range)
                    .service(iprange::add_ip_range)
                    .service(iprange::delete_ip_range)
                    .service(iprange::edit_ip_range)
                    .service(dhcprange::get_ip_range_dhcp)
                    .service(dhcprange::add_ip_range_dhcp)
                    .service(dhcprange::delete_ip_range_dhcp)
                    .service(dhcprange::edit_ip_range_dhcp)
                    .service(server::get_servers)
                    .service(server::add_server)
                    .service(server::delete_server)
                    .service(ddns::get_ddns)
                    .service(ddns::add_ddns)
                    .service(ddns::delete_ddns)
                    .service(dnszone::get_dns_zones)
                    .service(dnszone::add_dns_zone)
                    .service(dnszone::delete_dns_zone)
                    .service(dnszone::edit_dns_zone)
                    .service(dnsrecord::get_dns_zone_records)
                    .service(dnsrecord::add_dns_zone_record)
                    .service(dnsrecord::delete_dns_zone_record)
                    .service(dnsrecord::edit_dns_zone_record)
                    .service(group::get_groups),
            )
            .service(
                scope("/device")
                    .service(device::search_data)
                    .service(device::get_devices)
                    .service(device::get_device_info),
            )
            .service(
                scope("/agent")
                    .service(generate::get_server_roles)
                    .service(generate::generate_dns_data)
                    .service(generate::generate_dhcp_data)
                    .service(generate::success),
            )
            .service(scope("/datadog"))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi)),
    );
}

pub async fn get_app_data() -> Data<AppState> {
    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let jwt_cache: Arc<Mutex<HashMap<String, PKey<Public>>>> = Arc::new(Mutex::new(HashMap::new()));
    update_cache(jwt_cache.clone()).await.unwrap();
    Data::new(AppState {
        db: pool,
        jwt_cache,
    })
}
