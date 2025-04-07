import pytest
import faker
import inspect

from .scaling_splitter import UnicodePunctuationCamelSymbolSplitter

from hat_splitter import HATSplitter


def test_it_works() -> None:
    splitter = HATSplitter()
    assert splitter.split("hello world") == ["hello", "world"]


@pytest.mark.parametrize(
    "input",
    [
        "hello world",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam ac tempus ligula, sit amet tristique erat. Nulla mollis mauris ut magna aliquam tincidunt. Sed cursus arcu quam, nec tempor orci dignissim sit amet. Mauris a ultricies dui. Duis aliquet purus nec lectus volutpat dictum. Duis vel tempor nunc, eu vulputate arcu. Cras bibendum consequat facilisis. Pellentesque dolor velit, laoreet ac odio sit amet, malesuada fringilla nulla. Mauris efficitur erat et arcu sodales, in accumsan nulla pulvinar. Nulla nunc urna, fringilla id ligula ut, lobortis porttitor dolor. Phasellus placerat convallis pulvinar. Nullam ac fringilla sapien. Sed et turpis est. Ut suscipit hendrerit faucibus.",
        inspect.getsource(UnicodePunctuationCamelSymbolSplitter),
        "Um Kimchi-Jjigae zuzubereiten, erhitzen Sie 1 Esslöffel Pflanzenöl in einem großen Topf bei mittlerer Hitze. Fügen Sie 225 g dünn geschnittenen Schweinebauch oder Schulter hinzu und braten Sie ihn an, bis er gebräunt ist. Rühren Sie 2 Tassen gut fermentierten Kimchi ein und braten Sie es 3-4 Minuten, bis der Kimchi weich ist. Fügen Sie 4 Tassen Wasser oder Brühe, 1 Esslöffel Gochujang (koreanische rote Chilipaste) und 1 Esslöffel Sojasauce hinzu. Bringen Sie die Mischung zum Kochen, reduzieren Sie dann die Hitze und lassen Sie sie 20 Minuten köcheln. Fügen Sie 1 in Würfel geschnittenen Block Tofu hinzu und lassen Sie es weitere 5-10 Minuten köcheln. Servieren Sie den Eintopf heiß, garniert mit gehackten Frühlingszwiebeln und einer Schale gedämpftem Reis an der Seite."
        "反首再太清珠尾貫肖兌合馬玉人。連昔男封言圓干。上帽弟我頁往牠幼起友忍元丟豆三書走正四相。几話正見比對發飛。身安心木四巾向知刃欠他結讀反姊海路聲，旦員貓科。",
    ]
    + faker.Faker().texts(),
)
def test_it_matches_scaling_splitter(input: str) -> None:
    scaling_splitter = UnicodePunctuationCamelSymbolSplitter(max_chunk_size=64)
    splitter = HATSplitter()
    assert splitter.split_bytes(input) == scaling_splitter.split(input)
