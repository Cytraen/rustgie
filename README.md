# rustgie

[![Crates.io](https://img.shields.io/crates/v/rustgie?label=rustgie&style=flat-square)](https://crates.io/crates/rustgie)
[![Crates.io](https://img.shields.io/crates/v/rustgie_types?label=rustgie_types&style=flat-square)](https://crates.io/crates/rustgie_types)

A Bungie.net API client.

All requests *should* work.  
Requests that are part of the OAuth flow are not yet implemented.


## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rustgie::RustgieClientBuilder::new()
        .with_api_key("YOUR_API_KEY_HERE".to_string())
        .build()?;

    let manifest_response = client.destiny2_get_destiny_manifest(None).await?;
    println!("{:#?}", manifest_response.version.unwrap());

    let search_response = client.destiny2_search_destiny_player_by_bungie_name(
        rustgie_types::BungieMembershipType::All,
        rustgie_types::user::ExactSearchRequest {
            display_name: Some("Cytraen".parse().unwrap()),
            display_name_code: 2213
        }, None).await?;
    println!("{:#?}", search_response[0].display_name.as_ref().unwrap());

    Ok(())
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
