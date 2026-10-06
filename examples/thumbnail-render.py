import asyncio
from pathlib import Path

import mapli

CITIES = {
    "lisbon": (-9.14, 38.72),
    "porto": (-8.61, 41.15),
    "madrid": (-3.70, 40.42),
    "paris": (2.35, 48.86),
}


async def render_thumbnail(
    pool: mapli.RenderPool, name: str, lon: float, lat: float
) -> None:
    png_bytes = await pool.arender(
        "demo", lon=lon, lat=lat, zoom=5, width=256, height=256
    )
    Path(f"{name}.png").write_bytes(png_bytes)
    print(f"wrote {name}.png ({len(png_bytes)} bytes)")


async def main() -> None:
    pool = mapli.RenderPool(workers=2)
    pool.register_style("demo", url="https://demotiles.maplibre.org/style.json")

    await asyncio.gather(
        *(render_thumbnail(pool, name, lon, lat) for name, (lon, lat) in CITIES.items())
    )


if __name__ == "__main__":
    asyncio.run(main())
