# rustgie

[![rustgie crates.io](https://img.shields.io/crates/v/rustgie?label=rustgie&color=blue&logo=rust&style=flat-square)](https://crates.io/crates/rustgie)
[![rustgie_types crates.io](https://img.shields.io/crates/v/rustgie_types?label=rustgie_types&color=blue&logo=rust&style=flat-square)](https://crates.io/crates/rustgie_types)
[![minimum rustc: 1.60](https://img.shields.io/badge/minimum%20rustc-1.60-yellowgreen?logo=rust&style=flat-square)](https://www.whatrustisit.com)


A Bungie.net API client that does minimal wrapping and stays one-to-one with the [official API documentation.](https://bungie-net.github.io/multi/index.html)  
Still experimental, but it *should* fully work. Currently covers the full API surface of API v2.17.0 (2022-12-06, Season of the Seraph)

## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rustgie::RustgieClientBuilder::new()
        .with_api_key("YOUR_API_KEY_HERE")
        .build()?;

    let manifest_response = client.destiny2_get_destiny_manifest(None).await?;
    println!("{:#?}", manifest_response.version.expect("Manifest has no version"));

    let search_request_body = rustgie::types::user::ExactSearchRequest {
        display_name: Some("Cytraen".to_string()),
        display_name_code: 2213
    };

    let search_response = client.destiny2_search_destiny_player_by_bungie_name(
        rustgie::types::BungieMembershipType::All,
        search_request_body, None).await?;

    println!("{:#?}", search_response[0].display_name.as_ref().expect("No display name found"));

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
