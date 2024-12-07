use std::{future::{ready, Future, Ready}, pin::Pin, sync::{Arc, Mutex}, time::{Duration, Instant}};

use actix_web::{
    body::EitherBody, dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse
};
use moka::sync::Cache;

pub type LocalBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct RateLimiter
{
    pub cache: Arc<Mutex<Cache<String, (u64, Instant)>>>,
}

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
        
        
        ready(Ok(RateLimiterMiddleware { 
                service ,
                cache: Arc::clone(&self.cache),
            }))
    }
}

pub struct RateLimiterMiddleware<S> {
    service: S,
    cache: Arc<Mutex<Cache<String, (u64, Instant)>>>,
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
        let real_ip = String::from(req.connection_info().realip_remote_addr().unwrap_or_else(|| "unknown"));
        
        let ip = req .headers() .get("X-Forwarded-For") .and_then(|header_value| header_value.to_str().ok()) .map(|s| s.split(',').next().unwrap_or("unknown").trim()) .unwrap_or_else(|| &real_ip) .to_string();
        
        
        let cache = Arc::clone(&self.cache); 
        
        log::info!("cache {:#?}", cache); 
        
  
        let cache = cache.lock().unwrap();
        
        // Retrieve or initialize request count and timestamp
        let mut entry = cache.get(&ip).unwrap_or_else(|| (0, Instant::now()));

        // Reset count if time window expired (e.g., 60 seconds)
        if entry.1.elapsed() > Duration::new(3600, 0) {
            entry = (0, Instant::now());
        }

        // Check if the rate limit (100 requests) is exceeded
        if entry.0 >= 5 {
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
            Ok(res.map_into_left_body())
        })
    }
}

