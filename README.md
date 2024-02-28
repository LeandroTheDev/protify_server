# Protify Server

Creates the server for hosting the protify features

### Technical Informations

The servers by default is ready to receive 99 request from 1 ip during 1 minute, if a ip request more than this will be ignored without a response and reset the timer to 1 minute.

Body can only send 800 kilobytes, if a request send more than this it will be ignored with a Limit Overflow response.

Headers cannot be more than 255 kilobytes, if this is not satisfied the client will receive Limit Overflow response.
- All requests needs to include this headers: username, token, if not will get a Not Authorized response.
- anonymous user can send a request without token, but have limited access to actions in the server

Database Struct
- USERS

The database can be configured in src/components/database.rs
```
pub const DATABASE_NAME: &'static str = "protify_server";
pub const DATABASE_USERNAME: &'static str = "admin";
pub const DATABASE_PASSWORD: &'static str = "secret-password";
pub const DATABASE_IP: &'static str = "127.0.0.1";
pub const DATABASE_PORTS: u16 = 3306;
```

### Server Games

Protify launcher uses the zcat and tar linux to uncompress the game files using the command ``zcat game.tgz.part-* | tar -x``, the server needs to have a compressed file in parts using the command ``tar -cvzf - /path/to/folder | split -b 10m - game.``, the recommended size is 10 megabytes, the server send every part to the launcher and the launcher uncompress every part.

Default location for files, consider the path to the server
- Games: /store/games
- Softwares: /store/softwares

FTM License.