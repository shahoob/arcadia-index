# Testing

## Backend Testing

Adding additional tests to Arcadia is strongly encouraged, especially when adding new features! For unit tests, they can be added in the module being tested using standard rust idioms.

End-to-end tests can also be authored, they should be located in `tests/` and use the sqlx test fixture machinery to populate the database for testing. See `tests/test_auth.rs` for examples.

## Frontend Testing

We don't have any tests for the frontend. We'll add them once it's more "production ready".
