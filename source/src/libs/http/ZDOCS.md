## main
new -> returns a new instance for the http
infinity_listen -> start to listen every request to the address, needs a channel for send the requests, all requests will be handled already, and will return the HttpResponse with all data that you need to handle the request

this mean to be used only for very simple requests, do not use it for downloads, consider using sockets instead

### handle_http_stream
After listening the stream the handle_http_stream will manage the bytes received

Security Limit, the request should be at least 512 bytes, if not the buffer from the server will not read more and the data will be corrupted, with corrupted data the handle will probably terminate the connection and return a any internal error, because the headers is invalid, the returned error will be with code 400 and message: Invalid Headers

If the headers address is different from the true address receiving from the request the server will return the error code 400 and the message: Invalid Address

### status
Stores the enumeration for http status, simples say if the connection was interrupted during initialization

### response
Basic structure for receiving responses from the http instance, also include the methods enumeration for handling methods of requests
