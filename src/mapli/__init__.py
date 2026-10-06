"""mapli: Render MapLibre styles to images in-process.

>>> import asyncio
>>> from pathlib import Path
>>> import mapli
>>> CITIES = {
...     "lisbon": (-9.14, 38.72),
...     "porto": (-8.61, 41.15),
...     "madrid": (-3.70, 40.42),
...     "paris": (2.35, 48.86),
>>> }
>>> async def main() -> None:
...     pool = mapli.RenderPool(workers=2)
...     pool.register_style("demo", url="https://demotiles.maplibre.org/style.json")
...
...     async def thumbnail(name: str, lon: float, lat: float) -> None:
...         png = await pool.arender("demo", lon=lon, lat=lat, zoom=5, width=256, height=256)
...         Path(f"{name}.png").write_bytes(png)
...         print(f"wrote {name}.png ({len(png)} bytes)")
...
...     await asyncio.gather(
...         *(thumbnail(name, lon, lat) for name, (lon, lat) in CITIES.items())
...     )
...
>>> asyncio.run(main())
"""

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
    "RenderPool",
    "MapliError",
    "RenderError",
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
