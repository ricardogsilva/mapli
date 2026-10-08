import os
import subprocess
import sys

import pytest

SCRIPT = """
import mapli
try:
    mapli.RenderPool()
except mapli.GraphicsUnavailableError as exc:
    print(f"raised: {exc}")
"""


@pytest.mark.skipif(sys.platform != "linux", reason="the EGL check only runs on Linux")
def test_pool_creation_raises_when_egl_is_unavailable() -> None:
    # Point glvnd's EGL dispatcher at a vendor file that does not exist, so no EGL driver loads.
    # This runs in a subprocess because the check's result is cached for the whole process.
    env = {
        **os.environ,
        "__EGL_VENDOR_LIBRARY_FILENAMES": "/nonexistent/egl_vendor.json",
    }
    result = subprocess.run(
        [sys.executable, "-c", SCRIPT],
        env=env,
        capture_output=True,
        text=True,
        timeout=60,
        check=False,
    )
    # before the check existed, MapLibre Native aborted the process on the first render instead
    assert result.returncode == 0, result.stderr
    assert result.stdout.startswith("raised: no usable OpenGL (EGL) context")
