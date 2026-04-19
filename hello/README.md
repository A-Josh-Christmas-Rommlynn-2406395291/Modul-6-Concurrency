## **Commit 1 Reflection Notes**
### **The `handle_connection` function**
In the `handle_connection` function that has mutable parameter named `stream` of `TcpStream` data type, I create a new `BufReader` instance that wraps a reference to the stream. The `BufReader` adds buffering by managing calls to the `std::io::Read` trait methods for me. Then, I create a variable named `http_request` to collect the lines of the request the browser sends to our server. I indicate that I want to collect these lines in a vector by adding the `Vec<_>` type annotation. After that, print the read request(s).

## **Commit 2 Reflection Notes**
### **How `Content-Length` works?**
`Content-Length` header is set to the size of our response body. In this case, the size of `hello.html` (example file name). The `stream.write_all(response.as_bytes()).unwrap();` means:

• `response.as_bytes()` converts the `String` HTTP response into a byte slice, because network streams send bytes, not Rust strings.

• `stream.write_all(...)` writes all those bytes into the `TcpStream`, sending the response to the client.

• `.unwrap()` means: if writing fails, crash this function immediately with an error.
So, in summary, it sends the full HTTP response back through the socket, and panics if that send fails.

### **What does the `200 OK` status mean?**
The `200 OK` status means the webpage has requested successfully without any problems. It is partially different from `404 Not Found` that webpage requested successfully but has problem.

![Commit 2 screen capture](/hello/assets/images/commit2.png)

## **Commit 3 Reflection Notes**
### **Why I need to refactor?**
1. In line 11 on `main.rs`, `main()` manages server details directly. If there is configuration, logging, or thread pooling, this file quickly becomes *bloated*.
2. In line 24 on `main.rs`, `handle_connection()` parses requests with multiple `unwrap()`. This is fragile; invalid requests can cause server panic.
3. In line 33 on `main.rs`, `build_response()` mixes routing and file I/O. By design, route determination should be separated from how responses are constructed.
4. In line 34 on `main.rs`, routing is still hardcoded to a single path. As endpoints increase, this match becomes larger and more difficult to maintain.
5. In line 39 on `main.rs`, the file is read from disk with each request. This is fine for this exercise, but for better structure, the logic for mapping routes, responses, and file handling should usually be separated.
6. In line 12 on `main.rs` and line 30 on `main.rs`, error handling are inconsistent; there's `panic!` and `unwrap()`, so failure behavior is uncontrolled.
So, the answer is: `main.rs` needs refactoring, not because it doesn't work now, but because its structure isn't scalable, it's difficult to test each component, and it's too vulnerable to input errors.

## **Commit 4 Reflection Notes**
### **Why `thread::sleep` works in Rust?**
`thread::sleep` works here because it blocks the only thread that is handling both accepting requests and processing them. The flow is:
1. A request comes in.
2. The single main thread handles it.
3. If the request is `/sleep`, that same thread sleeps for 10 seconds.
4. During those 10 seconds, it cannot move to the next iteration of `listener.incoming()` and cannot handle another request.