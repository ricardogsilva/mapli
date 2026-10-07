"""mapli: Render MapLibre styles to images in-process."""

import json as _json
from os import PathLike
from typing import Any

from . import _core
from ._core import (
    MapliError,
    RenderError,
    StyleLoadError,
    UnknownStyleError,
)

__all__ = [
    "MapliError",
    "RenderError",
    "RenderPool",
    "StyleLoadError",
    "UnknownStyleError",
]


class RenderPool(_core.RenderPool):
    """A pool of renderers for MapLibre styles.

    This class is a thin wrapper around the core RenderPool class, providing
    a more user-friendly API for rendering MapLibre styles to images.
    """

    def register_style(
        self,
        style_id: str,
        *,
        url: str | None = None,
        path: str | PathLike[str] | None = None,
        json: str | dict[str, Any] | None = None,
    ) -> None:
        if isinstance(json, dict):
            json_str = _json.dumps(json)
            super().register_style(style_id, url=url, path=path, json=json_str)
        else:
            super().register_style(style_id, url=url, path=path, json=json)
