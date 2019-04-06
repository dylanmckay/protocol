# 2.0

  * Add a `Read` parameter to `HighLevel::from_low_level`

# 1.0

  * Remove the `define_composite_type!` and `define_packet_kind!` macros
    * Deprecated in favor of the #[derive] macro
  * Rename `Parcel::read` and `Parcel::write` to `read_field` and `write_field`
    * Aliases have been added for the `read` and `write` methods
  * Add a `hint::Hints` parameter to `Parcel`
  * Add a `logic::Aligned` type for automatic alignment
  * Add a new length prefix type `elements`. Length prefixes of this
    type specify the number of elements in a collection.


# 0.5

  * All enums now default to `#[protocol(discriminant = "string")]`
      * This discriminator type allows the most flexibility in future protocol changes.
      * The old behaviour can still be achieved via `#[protocol(discriminant = "integer")]`
  * Add `protocol::Settings` to Parcel API for byte order configuration
      * Every `Parcel` implementation must be adjusted to include the new argument
  * The `#[repr(integer type)]` enum attribute is now recognized and respected.

# 0.4 and below

Changelog did not exist when these versions were released.
