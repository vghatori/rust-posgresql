
use tower_http::cors::{CorsLayer, Any};

pub fn build_cors_permission(cors_config : Any) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(cors_config)
        .allow_headers(cors_config)
        .allow_methods(cors_config)
}
