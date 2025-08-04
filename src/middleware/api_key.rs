use actix_web::{
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;

#[derive(Clone)]
pub struct ApiKeyAuth {
    api_key: Rc<String>,
}

impl ApiKeyAuth {
    pub fn new(api_key: String) -> Self {
        ApiKeyAuth {
            api_key: Rc::new(api_key),
        }
    }
}

impl<S> Transform<S, ServiceRequest> for ApiKeyAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = ApiKeyAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(ApiKeyAuthMiddleware {
            service,
            api_key: self.api_key.clone(),
        })
    }
}

pub struct ApiKeyAuthMiddleware<S> {
    service: S,
    api_key: Rc<String>,
}

impl<S> Service<ServiceRequest> for ApiKeyAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let expected_key = self.api_key.clone();
        let api_key = req.headers().get("X-API-Key").and_then(|val| val.to_str().ok());

        if let Some(key) = api_key {
            if key == expected_key.as_str() {
                let fut = self.service.call(req);
                return Box::pin(async move { fut.await });
            }
        }

        let res = req.into_response(
            HttpResponse::Unauthorized()
                .json(serde_json::json!({ "error": "Unauthorized" }))
                .map_into_boxed_body(),
        );

        Box::pin(async move { Ok(res) })
    }
}
