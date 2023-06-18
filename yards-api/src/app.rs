use crate::{admin::*, models::*};

use actix_web::web::{self, scope, Data};

use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            server::get_servers,
            iprange::get_ip_range,
            dhcprange::get_ip_range_dhcp,
            ddns::get_ddns,
            dnszone::get_dns_zones,
            dnsrecord::get_dns_zone_records
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
        scope("/admin")
            .service(iprange::get_ip_range)
            .service(iprange::add_range)
            .service(dhcprange::get_ip_range_dhcp)
            .service(server::get_servers)
            .service(server::register_server)
            .service(ddns::get_ddns)
            .service(dnszone::get_dns_zones)
            .service(dnsrecord::get_dns_zone_records),
    )
    .service(scope("/agent"))
    .service(scope("/datadog"))
    .service(scope("/devices"))
    .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi));
}

pub async fn get_app_data() -> Data<AppState> {
    let pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    Data::new(AppState { db: pool })
}
