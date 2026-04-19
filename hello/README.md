## **Commit 1 Reflection Notes**
### The `handle_connection` function
In the `handle_connection` function that has mutable parameter named `stream` of `TcpStream` data type, I create a new `BufReader` instance that wraps a reference to the stream. The `BufReader` adds buffering by managing calls to the `std::io::Read` trait methods for me. Then, I create a variable named `http_request` to collect the lines of the request the browser sends to our server. I indicate that I want to collect these lines in a vector by adding the `Vec<_>` type annotation. After that, print the read request(s).

## **Commit 2 Reflection Notes**
### How `Content-Length` works?
`Content-Length` header is set to the size of our response body. In this case, the size of `hello.html` (example file name). The `stream.write_all(response.as_bytes()).unwrap();` means:

• `response.as_bytes()` converts the `String` HTTP response into a byte slice, because network streams send bytes, not Rust strings.

• `stream.write_all(...)` writes all those bytes into the `TcpStream`, sending the response to the client.

• `.unwrap()` means: if writing fails, crash this function immediately with an error.
So, in summary, it sends the full HTTP response back through the socket, and panics if that send fails.

### What does the `200 OK` status mean?
The `200 OK` status means the webpage has requested successfully without any problems. It is partially different from `404 Not Found` that webpage requested successfully but has problem.

![Commit 2 screen capture](/hello/assets/images/commit2.png)