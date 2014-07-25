extern crate iron;
extern crate router;
extern crate logger;
extern crate serialize;
extern crate postgres;
extern crate http;

// Chain trait required for .link() method
use iron::{Iron, Server, Chain};
use router::Router;
use logger::Logger;
use std::io::net::ip::Ipv4Addr;

mod dataset;
mod database;

fn main() {
    // Create the router
    let mut router = Router::new();

    // Pass the router to the various route creators (TODO)
    dataset::add_dataset_routes(&mut router);

    // Create the server
    let mut server: Server = Iron::new();

    // Enable logging
    let logger = Logger::new(None);
    server.chain.link(logger);

    // Add routes
    server.chain.link(router);

    // Start the server
    println!("Festivus server started on http://localhost:3000");
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
