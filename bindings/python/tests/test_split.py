import pytest
from hat_splitter import WhitespaceSplitter


def test_it_works():
    splitter = WhitespaceSplitter()
    assert splitter.split("hello world") == ["hello", "world"]


