use libphp::sapi::Sapi;

fn main() {
    let mut sapi = Sapi::new("ralph").expect("Failed to create SAPI.");
    
    sapi.startup();
    sapi.shutdown();
}
