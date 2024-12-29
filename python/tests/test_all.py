import pytest
import air_browser


def test_sum_as_string():
    assert air_browser.sum_as_string(1, 1) == "2"
