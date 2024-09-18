mod client;
mod server;

fn main() {

    let app = std::env::var("APP")
	.unwrap_or("".into());

    let receivers_nr = std::env::var("RCV")
	.ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    match app.as_str() {
	"S" => server::main(receivers_nr).unwrap(),
	"C" => client::main().unwrap(),
	_  => println!("$APP shall be `C' or `S'"),
    }
}
