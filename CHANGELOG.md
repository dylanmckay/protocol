# 0.5

  * All enums now default to `#[protocol(discriminant = "string")]`
      * This discriminator type allows the most flexibility in future protocol changes.
      * The old behaviour can still be achieved via `#[protocol(discriminant = "integer")]`
  * Add `protocol::Settings` to Parcel API for byte order configuration
      * Every `Parcel` implementation must be adjusted to include the new argument
  * The `#[repr(integer type)]` enum attribute is now recognized and respected.

# 0.4 and below

Changelog did not exist when these versions were released.
