use super::errors::Error;
use reqwest::header::{HeaderMap, HeaderName};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

#[inline]
pub fn load_file<S>(p: S) -> Result<Vec<u8>, Error>
where
    S: AsRef<str>,
{
    let p = p.as_ref();
    let f = File::open(p)?;
    let mut f = BufReader::new(f);
    let mut s = Vec::new();
    f.read_to_end(&mut s)?;
    Ok(s)
}

pub fn save_to_file<S>(c: Vec<u8>, p: S) -> Result<(), Error>
where
    S: AsRef<str>,
{
    let p = p.as_ref();
    let f = File::create(p)?;
    let mut f = BufWriter::new(f);
    f.write_all(&c)?;
    Ok(())
}

pub fn to_headers<S>(hashmap: HashMap<S, S>) -> Result<HeaderMap, Error>
where
    S: AsRef<str>,
{
    let mut headers = HeaderMap::new();
    for (key, val) in hashmap.iter() {
        let key = key.as_ref();
        let val = val.as_ref();
        headers.insert(HeaderName::from_bytes(key.as_bytes())?, val.parse()?);
    }
    Ok(headers)
}
