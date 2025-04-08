from abc import ABC, abstractmethod
from uniseg import wordbreak
import regex as re

# This implementation was copied from Scaling verbatim
# https://github.com/Aleph-Alpha/scaling-internal/blob/44a5c7286d01e288734f809bf43e821a8b6aa690/src/scaling/transformer/tokenizer/hierarchical.py#L252


class BaseTextSplitter(ABC):
    def __init__(self, max_chunk_size: int) -> None:
        """
        Initializes the BaseTextSplitter with the specified maximum chunk size in bytes.

        Args:
            max_chunk_size (int): The maximum size of each chunk in bytes.
        """
        self.max_chunk_size: int = max_chunk_size

    @abstractmethod
    def split(self, text: str) -> list[bytes]:
        """
        Splits the text into chunks of the specified maximum size.

        Args:
            text (str): The text to be split.

        Returns:
            List[bytes]: A list of chunks, where each chunk is a byte string.
        """
        raise NotImplementedError


class UnicodePunctuationCamelSymbolSplitter(BaseTextSplitter):
    def __init__(self, max_chunk_size: int) -> None:
        super().__init__(max_chunk_size)
        self.single_space = re.compile(r" ")
        self.all_whitespaces = re.compile(r"[\p{Z}\t\r\n\f]")
        self.punctuation = re.compile(r"([\p{P}])")
        self.camel_case = re.compile(r"(?<=\p{Ll})(?=\p{Lu})")

    def _split_punctuation(self, chunks: list[str]) -> list[str]:
        return [
            sub_chunk
            for chunk in chunks
            for sub_chunk in filter(None, self.punctuation.split(chunk))
        ]  # type: ignore

    def _split_camel_case(self, chunks: list[str]) -> list[str]:
        return [
            sub_chunk
            for chunk in chunks
            for sub_chunk in filter(None, self.camel_case.split(chunk))
        ]  # type: ignore

    def _merge_spaces(self, chunks: list[str]) -> list[str]:
        new_chunks: list[str] = []
        current_chunk = ""

        for chunk in chunks:
            if self.single_space.fullmatch(chunk):
                current_chunk += chunk
            else:
                if current_chunk:
                    new_chunks.append(current_chunk)
                    current_chunk = ""
                new_chunks.append(chunk)
        if current_chunk:
            new_chunks.append(current_chunk)
        return new_chunks

    def _prepend_spaces_and_append_punctuation(self, chunks: list[str]) -> list[str]:
        new_chunks: list[str] = []
        current_chunk = ""
        index = 0
        while index < len(chunks):
            # word consisting of only spaces
            if self.all_whitespaces.match(
                chunks[index]
            ) and not self.single_space.fullmatch(chunks[index]):
                new_chunks.append(chunks[index])
                index += 1
                continue

            ### build a word consisting of single space + word + punctuation
            # space
            if self.single_space.fullmatch(chunks[index]):
                current_chunk += chunks[index]
                index += 1
            # word
            if (
                index < len(chunks)
                and not self.all_whitespaces.match(chunks[index])
                and not self.punctuation.match(chunks[index])
            ):
                current_chunk += chunks[index]
                index += 1
            # punctuation
            while index < len(chunks) and self.punctuation.match(chunks[index]):
                current_chunk += chunks[index]
                index += 1

            if current_chunk:
                new_chunks.append(current_chunk)
                current_chunk = ""
        return new_chunks

    def _split_long_chunks(self, chunks: list[str]) -> list[bytes]:
        return [
            chunk.encode("utf-8")[i : i + self.max_chunk_size]
            for chunk in chunks
            for i in range(0, len(chunk), self.max_chunk_size)
        ]

    def split(self, text: str) -> list[bytes]:
        chunks = list(wordbreak.words(text))
        chunks = self._split_punctuation(chunks)
        chunks = self._split_camel_case(chunks)
        chunks = self._merge_spaces(chunks)
        chunks = self._prepend_spaces_and_append_punctuation(chunks)
        return self._split_long_chunks(chunks)
