use std::env;

use sift_rs::ping::v1::{PingRequest, ping_service_client::PingServiceClient};

mod grpc;
use grpc::{SiftChannelBuilder, config::SiftChannelConfig};

#[tokio::main]
async fn main() {
    let sift_creds = SiftChannelConfig {
        uri: env::var("SIFT_GRPC").expect("missing SIFT_GRPC env var"),
        apikey: env::var("SIFT_TOKEN").expect("missing SIFT_TOKEN env var"),
    };

    let sift_channel = SiftChannelBuilder::new(sift_creds).build();

    let mut ping_service = PingServiceClient::new(sift_channel);
    let res = ping_service
        .ping(PingRequest::default())
        .await
        .expect("failed to ping Sift");
    println!("{}", res.into_inner().response)
}
