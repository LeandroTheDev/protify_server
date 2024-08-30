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
To select specific from database use the:
```rust
let database_response: Vec<HashMap<String, String>> = database.select(
    vec![],
    "TABLENAME",
    vec![format!("ID = {}", item_id).as_str()],
    vec!["ID", "NAME", "CATEGORY", "LANGUAGES", "DESCRIPTION"],
);
```
This will select from table only with the same ``item_id``
#
