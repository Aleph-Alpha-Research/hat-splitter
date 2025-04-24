from typing import List, final

@final
class HatSplitter:
    """HatSplitter implements the HAT splitting rule.

    The HAT splitting rule is described in the
    [Hierarchical Autoregressive Transformers paper](https://arxiv.org/abs/2501.10322v2).
    You can use this to implement training and inference of HAT models.

    ```python
    from hat_splitter import HATSplitter

    my_hat_splitter = HATSplitter()
    words: list[str] = my_hat_splitter.split("Hello, world!")
    assert words == ["Hello,", " world!"]

    words: list[bytes] = my_hat_splitter.split_with_limit("Hello, world!", 4)
    assert words == [b'Hell', b'o,', b' wor', b'ld!']
    ```
    """

    def __init__(self) -> None: ...
    def split(self, text: str) -> List[str]:
        """Splits the input text into a list of strings based on the HAT splitting rule.

        Args:
            text (str): The input text to be split.

        Returns:
            List[str]: A list of words obtained by splitting the input text.
        """
        ...

    def split_with_limit(self, text: str, max_bytes_per_word: int) -> List[bytes]:
        """Splits a string into words and limits the size of each word to `max_bytes_per_word`.

        As this function enforces a byte limit, it may split unicode characters. That is,
        this function does not guarantee that the resulting byte arrays are valid UTF-8.

        Args:
            text (str): The input text to be split.
            max_bytes_per_word (int): The maximum number of bytes in a word.

        Returns:
            List[bytes]: A list of words (as bytes) obtained by splitting the input text.
        """
        ...
