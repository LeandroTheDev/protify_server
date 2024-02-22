# Protify Server

Creates the server for hosting the protify features

### Technical Informations

The servers by default is ready to receive 99 request from 1 ip during 1 minute, if a ip request more than this will be ignored without a response and reset the timer to 1 minute.

Body can only send 800 kilobytes, if a request send more than this it will be ignored with a Limit Overflow response.

Headers cannot be more than 255 kilobytes, if this is not satisfied the client will receive Limit Overflow response.
- All requests needs to include this headers: username, token, if not will get a Not Authorized response.

Default location for files, consider the path to the server
- Games: /store/games
- Softwares: /store/softwares

### Server Games

Protify launcher uses the zcat and tar linux to uncompress the game files using the command ``zcat game.tgz.part-* | tar -x``, the server needs to have a compressed file in parts using the command ``tar -cvzf - /path/to/folder | split -b 10m - game.``, the recommended size is 10 megabytes, the server send every part to the launcher and the launcher uncompress every part.

FTM License.