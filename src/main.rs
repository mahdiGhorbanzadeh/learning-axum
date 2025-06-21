
mod routes;
mod libs;

use libs::run;

#[tokio::main]
async fn main() {

    run().await;
}
