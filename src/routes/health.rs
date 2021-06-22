use super::*;
use warp::filters::BoxedFilter;

pub fn health_routes(
    client: db::Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Create the Health Check route
    health_check_route()
        .and(with_db(client))
        .and_then(health_handler)
        .with(warp::trace(|info| {
            // Construct our own custom span for this route.
            tracing::info_span!("Health Check", req.path = ?info.path())
        }))
}

pub fn health_check_route() -> BoxedFilter<()> {
    warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .boxed()
}

pub async fn health_handler(client: db::Client) -> Result<impl Reply, Rejection> {
    tracing::info!("Pinging Database");
    db::ping(&client)
        .await?;
    Ok(StatusCode::OK)
}

#[cfg(test)]
mod test {
    use super::*;
    use warp::test;

    #[tokio::test]
    async fn test_health_route() -> Result<(), String> {
        let filter = health_check_route();

        test::request()
            .path("/health")
            .method("GET")
            .filter(&filter)
            .await
            .map_err(|_| String::from("Health Route failted to match /health"))?;

        let result = test::request()
            .path("/healthy")
            .method("GET")
            .filter(&filter)
            .await;
        assert!(result.is_err()); // we should not match a rout that partially matches

        let result = test::request()
            .path("/health/more_health")
            .method("GET")
            .filter(&filter)
            .await;
        assert!(result.is_err()); // we should not match a route with additional path arguments

        Ok(())
    }
}