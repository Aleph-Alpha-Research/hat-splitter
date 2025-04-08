import pytest
import faker
# import inspect

from .scaling_splitter import UnicodePunctuationCamelSymbolSplitter

from hat_splitter import HATSplitter


@pytest.mark.parametrize(
    "text",
    [
        "hello world",
        "hello   world",
        "hello\tworld",
        "hello\nworld",
        "hello \n\nworld",
        "thisIsCamelCase and this is not",
        "punctuation!",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam ac tempus ligula, sit amet tristique erat. Nulla mollis mauris ut magna aliquam tincidunt. Sed cursus arcu quam, nec tempor orci dignissim sit amet. Mauris a ultricies dui. Duis aliquet purus nec lectus volutpat dictum. Duis vel tempor nunc, eu vulputate arcu. Cras bibendum consequat facilisis. Pellentesque dolor velit, laoreet ac odio sit amet, malesuada fringilla nulla. Mauris efficitur erat et arcu sodales, in accumsan nulla pulvinar. Nulla nunc urna, fringilla id ligula ut, lobortis porttitor dolor. Phasellus placerat convallis pulvinar. Nullam ac fringilla sapien. Sed et turpis est. Ut suscipit hendrerit faucibus.",
        "Um Kimchi-Jjigae zuzubereiten, erhitzen Sie 1 Esslöffel Pflanzenöl in einem großen Topf bei mittlerer Hitze. Fügen Sie 225 g dünn geschnittenen Schweinebauch oder Schulter hinzu und braten Sie ihn an, bis er gebräunt ist. Rühren Sie 2 Tassen gut fermentierten Kimchi ein und braten Sie es 3-4 Minuten, bis der Kimchi weich ist. Fügen Sie 4 Tassen Wasser oder Brühe, 1 Esslöffel Gochujang (koreanische rote Chilipaste) und 1 Esslöffel Sojasauce hinzu. Bringen Sie die Mischung zum Kochen, reduzieren Sie dann die Hitze und lassen Sie sie 20 Minuten köcheln. Fügen Sie 1 in Würfel geschnittenen Block Tofu hinzu und lassen Sie es weitere 5-10 Minuten köcheln. Servieren Sie den Eintopf heiß, garniert mit gehackten Frühlingszwiebeln und einer Schale gedämpftem Reis an der Seite.",
        # The following test cases don't work
        # "here! is some, more punctuation. I h0p3 thi$ isn't too 'much'.",
        # f"```python\n{inspect.getsource(UnicodePunctuationCamelSymbolSplitter)}\n```",
        # "国作", # This doesn't work as uniseg behaves differently to icu_segmenter :)
    ]
    + faker.Faker().texts(),
)
def test_it_matches_scaling_splitter(text: str) -> None:
    scaling_splitter = UnicodePunctuationCamelSymbolSplitter(max_chunk_size=64)

    splitter = HATSplitter()

    from pprint import pprint

    print(f"Input: {text}")

    print("Expected:")
    pprint(scaling_splitter.split(text))

    print("Actual:")
    pprint(splitter.split_bytes(text))

    assert splitter.split_bytes(text) == scaling_splitter.split(text)
