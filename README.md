```
% make py-hello-http
./target/release/wasmtime-test hello-http gen/hello-http.py.wasm
GET
AWAIT POLL
REGISTER
WAKERS [<hello_http.imports.poll.Pollable object at 0xaec090>] [<Future pending cb=[Task.task_wakeup()]>]
poll.poll
^Cmake: *** [py-hello-http] Interrupt: 2
```
