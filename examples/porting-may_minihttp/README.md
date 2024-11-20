# porting-may_minihttp

Porting of the crate [may_minihttp](https://crates.io/crates/may_minihttp) ([GitHub repository](https://github.com/Xudong-Huang/may_minihttp)).

## Usage

```js
const { HttpServer, HttpService, Request, Response } = require(".");

class HelloWorld extends HttpService {
    call(_req: Request, res: Response) {
        res.body("Hello, world!");
    }
}

const server = HttpServer(HelloWorld).start("0.0.0.0:8080").unwrap();

server.join().unwrap();
```
