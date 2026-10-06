"""mapli: Render MapLibre styles to images in-process."""

import asyncio
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

    async def arender(self, style_id: str, **kwargs: Any) -> bytes:
        """Asynchronously render a MapLibre style to an image.

        This method runs the synchronous `render` method in a separate thread,
        allowing it to be used in asynchronous contexts without blocking the event loop.
        """
        return await asyncio.to_thread(self.render, style_id, **kwargs)


#
# pool = mapli.RenderPool(workers=2)
# pool.register_style("demo", url="https://demotiles.maplibre.org/style.json")
# png_map = pool.render("demo", lon=-9.14, lat=38.72, zoom=4, width=512, height=512)
#
# and also an async wrapper:
#
# async def arender(self, *args, **kwargs) -> bytes:
#     return await asyncio.to_thread(self.render, *args, **kwargs)
