// https://github.com/gruberb/warp-examaple-api/blob/master/src/main.rs
// https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/


use warp::{http, Filter};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use warp::reply::json;


extern crate redis;
// use redis::Commands;

const MAX_PAYLOAD_SIZE: u64 = 1024 * 16;
const API_VERSION: &str = "v1";
const API_END_POINT: &str = "services";

type Items = HashMap<String, Item>;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    url: String,
    endpoints: Vec<String>,
    authorized_roles: Vec<String>,
}

#[derive(Clone)]
struct Store {
  service_list: Arc<RwLock<Items>>
}

impl Store {
    fn new() -> Self {
        Store {
            service_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn add_service_list_item(
    item: Item,
    _store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {

        
        /* store.service_list.write().insert(item.name.to_string(), Item {
            name: String::from(item.name),
            url: String::from(item.url),
            endpoints: item.endpoints,
            authorized_roles: item.authorized_roles,
        }); */

        let mut conn = connect();


        let mut hash: BTreeMap<String, _> = BTreeMap::new();
        // let mut hash = BTreeMap::new();
        let prefix = "redis-hash";
        /* hash.insert(String::from("name"), Item {
            name: String::from(item.name),
            url: String::from(item.url),
            endpoints: item.endpoints,
            authorized_roles: item.authorized_roles,
        }); */ 
        // hash.insert(String::from("name"), String::from(item.name));
        hash.insert(String::from("url"), Item {
            name: String::from(item.name),
            url: String::from(item.url),
            endpoints: item.endpoints,
            authorized_roles: item.authorized_roles,
        });
        // hash.insert(String::from("endpoints"), item.endpoints);
        /* 
         hash.insert(item.name.to_string(), Item {
            name: String::from(item.name),
            url: String::from(item.url),
            endpoints: item.endpoints,
            authorized_roles: item.authorized_roles,
        });
        */

        let _: () = redis::cmd("HSET")
            .arg(format!("{}:{}", prefix, "rust1"))
            .arg(hash)
            .query(&mut conn)
            .expect("failed to execute HSET");

        let info: BTreeMap<String, String> = redis::cmd("HGETALL")
            .arg(format!("{}:{}", prefix, "rust1"))
            .query(&mut conn)
            .expect("failed to execute HGETALL");
        println!("info for rust redis hash: {:?}", info);


        // Ok(warp::reply::with_status(json(&info), http::StatusCode::CREATED));
        Ok(warp::reply::with_status(
            json(&info),
            http::StatusCode::CREATED,
        ))
}

async fn get_service_list(
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {

    let mut result = HashMap::new();
    let r = store.service_list.read();
    // println!("{:?}", r);
    for (key, value) in r.iter() {
        result.insert(key, value);
        // println!("{:?}", value);
    }

    Ok(warp::reply::json(
        &result
    ))
}


fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(MAX_PAYLOAD_SIZE).and(warp::body::json())
}

fn connect() -> redis::Connection {
    //format - host:port
    /* let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    //if Redis server needs secure connection
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name); */
    redis::Client::open("redis://127.0.0.1/")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

/* fn basics() {
    let mut conn = connect();
    let _: () = redis::cmd("SET")
        .arg("foo")
        .arg("bar")
        .query(&mut conn)
        .expect("failed to execute SET for 'foo'");
    
        let bar: String = redis::cmd("GET")
        .arg("foo")
        .query(&mut conn)
        .expect("failed to execute GET for 'foo'");
    println!("value for 'foo' = {}", bar);
    
    let _: () = conn
        .incr("counter", 2)
        .expect("failed to execute INCR for 'counter'");
    let val: i32 = conn
        .get("counter")
        .expect("failed to execute GET for 'counter'");
    println!("counter = {}", val);
}


fn hash() {
    let mut conn = connect();
    
    let mut hash: BTreeMap<String, String> = BTreeMap::new();
    let prefix = "redis-hash";
    hash.insert(String::from("name"), String::from("redis-rs"));
    hash.insert(String::from("version"), String::from("0.19.0"));
    hash.insert(
        String::from("repo"),
        String::from("https://github.com/mitsuhiko/redis-rs"),
    );

    let _: () = redis::cmd("HSET")
        .arg(format!("{}:{}", prefix, "rust"))
        .arg(hash)
        .query(&mut conn)
        .expect("failed to execute HSET");
    let info: BTreeMap<String, String> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, "rust"))
        .query(&mut conn)
        .expect("failed to execute HGETALL");
    println!("info for rust redis hash: {:?}", info);
    let _: () = conn
        .hset_multiple(
            format!("{}:{}", prefix, "go"),
            &[
                ("name", "go-redis"),
                ("version", "8.4.6"),
                ("repo", "https://github.com/go-redis/redis"),
            ],
        )
        .expect("failed to execute HSET");
    let repo_name: String = conn
        .hget(format!("{}:{}", prefix, "go"), "repo")
        .expect("HGET failed");
    println!("go redis hash repo name: {:?}", repo_name);
}*/

#[tokio::main]
async fn main() {

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_services = warp::post()
        .and(warp::path(API_VERSION))
        .and(warp::path(API_END_POINT))
        .and(warp::path::end())
        .and(json_body())
        .and(store_filter.clone())
        .and_then(add_service_list_item);

    let get_services = warp::get()
        .and(warp::path(API_VERSION))
        .and(warp::path(API_END_POINT))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_service_list);

    let routes = add_services.or(get_services);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
