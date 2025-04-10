import pytest
import faker
import inspect

from .scaling_splitter import UnicodePunctuationCamelSymbolSplitter

from hat_splitter import HATSplitter


WORD_BYTES_LIMIT = 64


def read_shakespeare_text():
    with open("../../data/shakespeare.txt", "r") as f:
        text = f.read()
    return text


@pytest.fixture
def shakespeare_text():
    return read_shakespeare_text()


FAKER_TEXT_COUNT = 5


@pytest.mark.parametrize(
    "text",
    [
        "hello world",
        "hello   world",
        "hello\tworld",
        "hello\nworld",
        "hello \n\nworld",
        "thisIsCamelCase and this is not",
        "this_is_a_long_snake_case_word",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "Hello, world!",
        "0123456789",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam ac tempus ligula, sit amet tristique erat. Nulla mollis mauris ut magna aliquam tincidunt. Sed cursus arcu quam, nec tempor orci dignissim sit amet. Mauris a ultricies dui. Duis aliquet purus nec lectus volutpat dictum. Duis vel tempor nunc, eu vulputate arcu. Cras bibendum consequat facilisis. Pellentesque dolor velit, laoreet ac odio sit amet, malesuada fringilla nulla. Mauris efficitur erat et arcu sodales, in accumsan nulla pulvinar. Nulla nunc urna, fringilla id ligula ut, lobortis porttitor dolor. Phasellus placerat convallis pulvinar. Nullam ac fringilla sapien. Sed et turpis est. Ut suscipit hendrerit faucibus.",
        "Um Kimchi-Jjigae zuzubereiten, erhitzen Sie 1 Esslöffel Pflanzenöl in einem großen Topf bei mittlerer Hitze. Fügen Sie 225 g dünn geschnittenen Schweinebauch oder Schulter hinzu und braten Sie ihn an, bis er gebräunt ist. Rühren Sie 2 Tassen gut fermentierten Kimchi ein und braten Sie es 3-4 Minuten, bis der Kimchi weich ist. Fügen Sie 4 Tassen Wasser oder Brühe, 1 Esslöffel Gochujang (koreanische rote Chilipaste) und 1 Esslöffel Sojasauce hinzu. Bringen Sie die Mischung zum Kochen, reduzieren Sie dann die Hitze und lassen Sie sie 20 Minuten köcheln. Fügen Sie 1 in Würfel geschnittenen Block Tofu hinzu und lassen Sie es weitere 5-10 Minuten köcheln. Servieren Sie den Eintopf heiß, garniert mit gehackten Frühlingszwiebeln und einer Schale gedämpftem Reis an der Seite.",
        "here! is some, more punctuation. I h0p3 thi$ isn't too 'much'.{{}}()<>$%^&*",
        read_shakespeare_text(),
        f"```python\n{inspect.getsource(UnicodePunctuationCamelSymbolSplitter)}\n```",
        "足王食皮已「住看長蝶視昔次金肖雞」元男躲秋快都同現正！見一內聽故耍了唱登吉了結。只菜就秋。息前冒家冬英百海百遠澡意比，念道話豆流用人？高老信乙即服你京秋別點口邊木意夏青扒布足，口久姊爪三村什誰蛋對。",
    ]
    + faker.Faker().texts(nb_texts=FAKER_TEXT_COUNT),
    ids=[
        "simple",
        "multiple_spaces",
        "tab",
        "newline",
        "multiple_newlines",
        "camel_case",
        "snake_case",
        "long_word",
        "punctuation",
        "numbers",
        "lorem",
        "german_text",
        "punctuation_with_symbols",
        "shakespeare_text",
        "python_snippet",
        "chinese_text",
    ]
    + [f"faker_text_{i}" for i in range(FAKER_TEXT_COUNT)],
)
def test_it_matches_scaling_splitter(text: str) -> None:
    expected = UnicodePunctuationCamelSymbolSplitter(
        max_chunk_size=WORD_BYTES_LIMIT
    ).split(text)
    actual = HATSplitter().split_with_limit(text, WORD_BYTES_LIMIT)

    assert actual == expected


def test_benchmark_hat_splitter(benchmark, shakespeare_text):
    splitter = HATSplitter()

    benchmark(splitter.split_with_limit, shakespeare_text, WORD_BYTES_LIMIT)


def test_benchmark_scaling_splitter(benchmark, shakespeare_text):
    splitter = UnicodePunctuationCamelSymbolSplitter(max_chunk_size=WORD_BYTES_LIMIT)

    benchmark(splitter.split, shakespeare_text)
