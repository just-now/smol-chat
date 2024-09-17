mod client;
mod server;
use smol::io;

fn main() -> io::Result<()> {
    match std::env::var("APP") {
        Ok(val) => {
	    match val.as_str() {
		"S" => server::main(),
		"C" => client::main(),
		_ => {
		    println!("$APP shall be `C' or `S'");
		    Ok(())
		},
	    }
	},
        Err(e) => {
	    println!("couldn't interpret $APP, {e}");
	    Ok(())
	},
    }
}
