use std::fs::File;
use std::io::{ Write, Error };

use rocketsim::physics::earth::atmosphere;

fn main() -> Result<(), Error> {
    let mut file = match  File::create("atmosphere.csv") {
        Err(msg) => panic!("Cannot create file: {}", msg),
        Ok(file) => file
    };
    
    writeln!(file, "altitude,temperature,pressure,density")?;

    for altitude in 1..86001 {
        let values = atmosphere::atmosisa(altitude as f32);

        writeln!(file, "{},{},{},{}", altitude, values.temperature, values.pressure, values.density)?;
    }

    Ok(())
}
