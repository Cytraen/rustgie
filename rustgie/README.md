# rustgie

A Bungie.net API client that does minimal wrapping and stays one-to-one with the [official API documentation.](https://bungie-net.github.io/multi/index.html)  
Still experimental, but it *should* fully work. Currently covers the full API surface of API v2.15.0 (Season of the Haunted)


## Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = rustgie::RustgieClientBuilder::new()
        .with_api_key("YOUR_API_KEY_HERE")
        .build()?;

    let manifest_response = client.destiny2_get_destiny_manifest(None::<&str>).await?;
    println!("{:#?}", manifest_response.version.expect("Manifest has no version"));

    let search_request_body = rustgie_types::user::ExactSearchRequest {
        display_name: Some("Cytraen".to_string()),
        display_name_code: 2213
    };

    let search_response = client.destiny2_search_destiny_player_by_bungie_name(
        rustgie_types::BungieMembershipType::All,
        search_request_body, None::<&str>).await?;

    println!("{:#?}", search_response[0].display_name.as_ref().expect("No display name found"));

    Ok(())
}
```

## Differences vs. official documentation

- Bitmask/flag enums that are represented as signed integers are now represented as unsigned
- Bitmask/flag enum values with zero/multiple bits set (e.g. `None`, `All`) have been removed
- Argument/property/etc. names have been changed from `camelCase` to `snake_case` to fit Rust convention

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
