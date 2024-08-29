# Database
Creates a connection to the desired database to start the connection use the command ``let database_result = Database::new()``

Get the result by using the ``match database_result``
```rust
match database_result {
    Ok(database) {

    }
    Err(error) {

    }
}
```

#
To use the select to database use the:
```rust
let database_response: Vec<HashMap<String, String>> = database.select(vec![], "TABLENAME", vec![], vec!["ID, USER, PASSWORD"]);
```
This is select from table ALL [ID, USER, PASSWORD]
#
