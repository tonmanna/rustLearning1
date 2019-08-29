use std::error::Error;
use std::io;

pub fn readCSV(msg: String) -> Result<(), Box<dyn Error>> {
    let reader = io::stdin();
    let mut rdr = csv::Reader::from_path(msg)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
