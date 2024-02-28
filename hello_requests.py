import asyncio
import hello_poll_loop

from hello_http.imports.types import (
    Scheme, SchemeHttp, SchemeHttps, SchemeOther,
    Fields, OutgoingRequest
)
from hello_poll_loop import Stream, PollLoop
from urllib import parse

class Response(object):

    def __init__(self, code):
        self.code = code


def get(url: str) -> Response:
    loop = asyncio.get_event_loop()
    t = loop.create_task(handle_get(url))
    print("GET")
    return loop.run_until_complete(t)

async def handle_get(url: str) -> Response:
    """Download the contents of the specified URL, computing the SHA-256
    incrementally as the response body arrives.

    This returns a tuple of the original URL and either the hex-encoded hash or
    an error message.
    """
    url_parsed = parse.urlparse(url)

    match url_parsed.scheme:
        case "http":
            scheme: Scheme = SchemeHttp()
        case "https":
            scheme = SchemeHttps()
        case _:
            scheme = SchemeOther(url_parsed.scheme)

    request = OutgoingRequest(Fields.from_list([]))
    request.set_scheme(scheme)
    request.set_authority(url_parsed.netloc)
    request.set_path_with_query(url_parsed.path)

    print("AWAIT POLL")
    response = await hello_poll_loop.send(request)
    status = response.status()
    resp = Response(status)
    print("ASYNC", resp)
    return resp

    # if status < 200 or status > 299:
    #     return url, f"unexpected status: {status}"

    # stream = Stream(response.consume())
    # hasher = hashlib.sha256()
    # while True:
    #     chunk = await stream.next()
    #     if chunk is None:
    #         return url, hasher.hexdigest()
    #     else:
    #         hasher.update(chunk)

