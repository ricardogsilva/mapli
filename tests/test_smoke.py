import pytest

import mapli


def test_compiled_extension_is_importable():
    assert issubclass(mapli.RenderPool, mapli._core.RenderPool)


@pytest.mark.parametrize(
    "error_class",
    [mapli.UnknownStyleError, mapli.StyleLoadError, mapli.RenderError],
)
def test_errors_share_base_class(error_class):
    assert issubclass(error_class, mapli.MapliError)
