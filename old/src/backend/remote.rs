use std::io::{BufRead, BufReader, Read};

use crate::error::Error;

pub fn fetch_package_list() -> Result<Vec<String>, Error> {
    const URL: &str = "https://raw.githubusercontent.com/jwpjrdev/winter/master/package_list";
    let data = match ureq::get(URL).call() {
        Ok(data) => data,
        Err(err) => return Err(Error::FailedRequest(err)),
    };

    let buf = BufReader::new(data.into_reader());

    Ok(parse_list(buf)?)
}

fn parse_list(list: BufReader<Box<dyn Read + Send + Sync>>) -> Result<Vec<String>, Error> {
    let mut result = vec![];
    
    for l in list.lines() {
        let line = l.unwrap;
        if !line.starts_with("#") {
            if !line.trim().is_empty() {
                let values: Vec<String> = line.split(",")
                    .map(|s| s.to_string())
                    .collect();
                result.extend(values);
            }
        }
    }

    result
}