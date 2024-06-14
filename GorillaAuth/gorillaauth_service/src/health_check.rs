use axum::response::IntoResponse;
use serde_json;

use crate::responses::*;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Health Check";

    let response = HealthCheckResponse {
        status: ResponseStatus::Success,
        message: MESSAGE.to_string(),
    };

    serde_json::to_string(&response).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode}
    };
    use http_body_util::BodyExt; 
    use tower::ServiceExt;
    use crate::app;

    #[tokio::test]
    async fn test_health_check_handler() {
        let routes = app();

        let response = routes
            .oneshot(Request::builder().uri("/api/health_check").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: HealthCheckResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.status, ResponseStatus::Success);
        assert_eq!(body.message, "Health Check");
    }
}