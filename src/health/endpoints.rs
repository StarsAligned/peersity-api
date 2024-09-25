use actix_web::{get, web::ServiceConfig, HttpResponse};

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(get_health);
    cfg.service(get_health_version);
}

#[get("/health")]
async fn get_health() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

#[get("/health/version")]
async fn get_health_version() -> HttpResponse {
    return HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use crate::health::endpoints::{get_health, get_health_version};

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(get_health)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_health_version() {
        let app = test::init_service(App::new().service(get_health_version)).await;
        let req = test::TestRequest::get().uri("/health/version").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, format!(env!("CARGO_PKG_VERSION")));
    }
}