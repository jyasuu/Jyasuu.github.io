use std::{future::{ready, Future, Ready}, pin::Pin, time::{Duration, Instant}};

use actix_web::{
    body::EitherBody, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse
};
use moka::sync::Cache;

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RateLimiter;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for RateLimiter
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = RateLimiterMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        
        let cache : Cache<String, (u64, Instant)> = Cache::new(1000);
        ready(Ok(RateLimiterMiddleware { service ,cache}))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    cache : Cache<String, (u64, Instant)>,
}

impl<S, B> Service<ServiceRequest> for RateLimiterMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract client IP from `X-Forwarded-For` header, fallback to `realip`
        let ip = String::from("127.0.0.1");
        let cache = &self.cache;

        log::info!("cache {:#?}",cache);

        // Retrieve or initialize request count and timestamp
        let mut entry = cache.get(&ip).unwrap_or_else(|| (0, Instant::now()));

        // Reset count if time window expired (e.g., 60 seconds)
        if entry.1.elapsed() > Duration::new(60, 0) {
            entry = (0, Instant::now());
        }

        // Check if the rate limit (100 requests) is exceeded
        if entry.0 >= 100 {
            return Box::pin(async {
                Ok(req.into_response(
                    HttpResponse::TooManyRequests()
                        .finish()
                        .map_into_right_body(),
                ))
            });
        }

        // Update the cache with incremented request count
        cache.insert(ip.clone(), (entry.0 + 1, entry.1));

        


        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res.map_into_left_body())
        })
    }
}
