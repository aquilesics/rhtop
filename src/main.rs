use std::sync::{Arc, Mutex};

use axum::{routing::get,Router,Server, extract::State};
use sysinfo::{System, SystemExt, CpuExt};

#[tokio::main]
async fn main() {
    let router =
        Router::new()
        .route("/",get(root_get))
        .route("/api/cpus",get(cpus_get))
        .with_state(AppState {
             sys: Arc::new( Mutex::new(System::new()))
             });    

    let server = 
        Server::bind(&"0.0.0.0:7032"
                        .parse()
                        .unwrap())
                        .serve(router.into_make_service());

    let addr = server.local_addr();
    println!("linsten on {addr}");
    
    server.await.unwrap(); 
}

#[derive(Clone)]
struct AppState{
    sys:Arc<Mutex<System>>,
}

async fn root_get() -> &'static str {
    "Hello"
   
}

async fn cpus_get(State(state):State<AppState>) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    let mut sys = state.sys.lock().unwrap();
    // let  mut sys = System::new();
    sys.refresh_cpu();
    for ( i, cpu ) in sys.cpus().iter().enumerate(){
        let i = i + 1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s,"CPU {i} {usage}%").unwrap();
        
    }
    s
}