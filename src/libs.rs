
use axum::Router;
use tokio::net::TcpListener;

use crate::routes::routes;

pub async fn run(){

    let app:Router = routes().await;

    match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => {
            axum::serve(listener,app).await.unwrap();
        },

        Err(e) => {
            println!("Error in listener {} ",e);    
        }       
    }
}