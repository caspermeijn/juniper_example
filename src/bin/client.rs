//! An example of creating a connection and running a single subscription on it
//! using `graphql-client` and `async-tungstenite`
//!
//! Talks to the the tide subscription example in `async-graphql`

use futures::StreamExt;
use graphql_client::GraphQLQuery;
use graphql_ws_client::graphql::StreamingOperation;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/client.graphql",
    schema_path = "src/server_schema.graphql",
    response_derives = "Debug"
)]
struct Count;

#[tokio::main]
async fn main() {
    use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};
    use graphql_ws_client::Client;

    let mut request = "ws://127.0.0.1:8080/subscriptions".into_client_request().unwrap();
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
    );

    let (connection, _) = async_tungstenite::tokio::connect_async(request)
        .await
        .unwrap();

    println!("Connected");

    let mut subscription = Client::build(connection)
        .subscribe(StreamingOperation::<Count>::new(
            count::Variables,
        ))
        .await
        .unwrap();

    while let Some(item) = subscription.next().await {
        println!("{item:?}");
    }
}