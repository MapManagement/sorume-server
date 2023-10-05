# Logging Guidelines

This document defines the log level for certain operations withint the backend.

Following shorthands are used for different types of operations:

- ``C`` - create
- ``R`` - read
- ``U`` - update
- ``D`` - delete

## Debug

- reading an object

```rust
```

- invalid id when reading an object

```rust
```

## Information

- successful creation operation

```rust

Ok(profile) => {
    info!("C: New profile has been created: {:?}", profile.profile_id);
    return Ok(profile);
}
```

## Warning

- failed create operation

```rust
Err(err) => {
    warn!("C: Unable to create a new profile: {}", err);
    return Err(err);
}

```

- invalid id when updating or deleting an object

```rust
warn!("U: Profile with ID {:?} does not exist", profile_id);
```

- failed update operation

```rust
Err(err) => {
    warn!("U: Unable to update profile: {}", err);
    return Err(err);
}
```

- not enough objects

```rust
if member_ids.len() < 1 {
    warn!("C: Cannot create a group chat with less than one member");

    return Err(DbErr::Custom(
        "A group chat needs at least one member.".to_owned(),
    ));
}
```

## Error

N/A

## Trace

N/A
