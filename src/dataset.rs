use iron::{Iron, Server, Middleware, Request, Response, Alloy, FromFn};
use iron::{Status, Continue, Unwind};
use router::Router;
use http::method::{Get, Post, Put, Patch, Delete};

use serialize::json;

use database::get_db_conn;

#[deriving(Decodable, Encodable)]
struct Dataset {
    name: String,
    description: String,
    unit: String
}

pub fn add_dataset_routes(router: &mut Router) {
    let route = "/datasets";
    router.route(Get, route.to_string(), vec![], FromFn::new(get_datasets));
}

fn get_datasets(req: &mut Request, res: &mut Response, alloy: &mut Alloy) -> Status {
    let conn = match get_db_conn() {
        Ok(c) => c,
        Err(_) => return Unwind
    };

    let statement = conn.prepare("SELECT name, description, unit FROM datasets").unwrap();
    for row in statement.query([]).unwrap() {
        let dataset = Dataset {
            name: row.get("name"),
            description: row.get("description"),
            unit: row.get("unit")
        };
        res.write(json::encode(&dataset).as_bytes());
    }
    Continue
}
