from utils.foo import bar
import json


def run(*args, **kwargs):
    return json.dumps({
        "score": args[0]
    })
