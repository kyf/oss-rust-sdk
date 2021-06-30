use oss_rust_sdk::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let oss_instance = OSS::new(
        "your_AccessKeyId",
        "your_AccessKeySecret",
        "your_Endpoint",
        "your_Bucket",
    );

    oss_instance.put_object_from_file("example", "example", None::<HashMap<&str, &str>>, None).await?;

    let buf = oss_instance.get_object("cargo-toml", None::<HashMap<&str, &str>>, None).await?;
    println!("buffer = {:?}", String::from_utf8(buf.to_vec()));

    oss_instance.get_object_to_file("example.download1", "example", None::<HashMap<&str, &str>>, None).await?;
    Ok(())
}
