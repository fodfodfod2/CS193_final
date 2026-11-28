#!/bin/bash

uv init # Create a new venv if not there already for some reason
uv add laya
uv add "laya[serve]"
LAYA_HOST=127.0.0.1 LAYA_MODELS=english uv run laya-serve
