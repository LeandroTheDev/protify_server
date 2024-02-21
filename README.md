# Protify Server

Creates the server for hosting the protify features

### Technical Informations

The servers by default is ready to receive 99 request from 1 ip during 1 minute, if a ip request more than this will be ignored without a response.

Body can only send 10 megabytes, if a request send more than this it will be ignored with a Limit Overflow response.

Headers cannot be empty the server is waiting for more than 0 bytes from the header, if this is not satisfied the client will receive Limit Overflow response.
- What need to be included in headers: id, token, username.