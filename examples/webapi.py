"""Example FastAPI web API that uses mapli to render map images."""

from contextlib import asynccontextmanager
from typing import (
    Annotated,
    Literal,
)

from fastapi import (
    FastAPI,
    HTTPException,
    Query,
)
from fastapi.responses import Response

import mapli

_STYLES = {
    "demo": "https://demotiles.maplibre.org/style.json",
    "liberty": "https://tiles.openfreemap.org/styles/liberty",
    "positron": "https://tiles.openfreemap.org/styles/positron",
}


@asynccontextmanager
async def lifespan(app: FastAPI):
    pool = mapli.RenderPool(workers=4)
    for id_, url in _STYLES.items():
        pool.register_style(id_, url=url)
    app.state.mapli_render_pool = pool
    yield
    del app.state.pool  # dropping the pool shuts down the workers


app = FastAPI(lifespan=lifespan)


@app.get(
    "/maps/{style_id}",
    response_model=None,
    response_class=Response,
    responses={
        "200": {
            "content": {
                "image/png": {},
            }
        }
    },
)
async def render_map(
    style_id: Literal["demo", "liberty", "positron"] = "demo",
    lon: Annotated[float, Query(ge=-180, le=180)] = -9.14,
    lat: Annotated[float, Query(ge=-85.05, le=85.05)] = 38.72,
    zoom: Annotated[float, Query(ge=0, le=24)] = 3.0,
    width: Annotated[int, Query(ge=1, le=2048)] = 512,
    height: Annotated[int, Query(ge=1, le=2048)] = 512,
) -> Response:
    """Return a map image in PNG format."""
    try:
        png_bytes = await app.state.mapli_render_pool.arender(
            style_id, lon=lon, lat=lat, zoom=zoom, width=width, height=height
        )
    except mapli.UnknownStyleError:
        raise HTTPException(status_code=404, detail=f"unknown style {style_id!r}")
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except mapli.MapliError as e:
        raise HTTPException(status_code=502, detail=str(e))
    return Response(content=png_bytes, media_type="image/png")
