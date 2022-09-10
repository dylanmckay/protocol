# 3.4.0

  * Re-export the `#[derive(Protocol)]` attribute directly from the `protocol` crate
    so that only one dependency needs to be pulled in. This feature is enabled by
    default and can be opted out with `default-features = false`.

# 3.3.0
  * Deprecate the `HighLevel` trait as it is horribly designed and often causes
    conflicting blanket impl errors in downstream crates that also attempt
    blanket `Parcel` impls.

# 3.1.1
  * Implement `Parcel` for tuple 5-tuples, 6-tuples, all the way to 10-tuples

# 3.0
  * Change the order of parameters to the list writer methods in the 'util' module
    ** The previous ordering was confusing, and also inconsistent with the ordering to `util::read_*`.


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
