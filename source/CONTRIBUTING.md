# Understanding
### Infinite Functions
Functions that will freeze the server if not called in a secondadry thread ``thread::spawn``, generally with a loop

### Handling Functions
Every step you do in the function that return something that needs to receive a treatment will need a handling function,
for example listening any TcpStream after a success listening and received the stream we need to handle the message from the stream.

### Creating a new struct for handling something