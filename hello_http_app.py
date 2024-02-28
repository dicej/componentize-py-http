"""Demo of a serverless app using `wasi-http` to handle inbound HTTP requests.

This demonstrates how to use WASI's asynchronous capabilities to manage multiple
concurrent requests and streaming bodies.  It uses a custom `asyncio` event loop
to thread I/O through coroutines.
"""
import asyncio

import hello_poll_loop
import hello_http
import hello_requests

# from hello_http import exports
# 
# 
# class Run(exports.Run):
#     def run(self):
#         loop = hello_poll_loop.PollLoop()
#         asyncio.set_event_loop(loop)
# 
#         r = hello_requests.get("https://example.com")
#         value = f"status: {r.code}"
#         print(value)

class HelloHttp(hello_http.HelloHttp):
    def hello(self, name) -> str:
        loop = hello_poll_loop.PollLoop()
        asyncio.set_event_loop(loop)

        r = hello_requests.get("https://example.com")
        value = f"status: {r.code}"
        print(value)
        return value
