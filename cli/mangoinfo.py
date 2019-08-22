"""
Mango CLI Tools
Copyright (C) 2019  Alex Fence

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <http://www.gnu.org/licenses/>.
"""
import click

import mangofmt
from mangofmt import MangoFile


class MetadataPair():
    def __init__(self, key, value):
        self._key = key

        if value is not None:
            self._value = value
        else:
            self._value = ""

    @property
    def key(self):
        return self._key

    @property
    def value(self):
        return self._value


class ImagePair(MetadataPair):
    pass


meta_keys = [
    "title",
    "author",
    "publisher",
    "source",
    "translation",
    "language",
    "volume",
    "chapter",
    "year"
]


def get_meta_pairs(meta: mangofmt.mango.MangoMetaData, verbose):
    pairs = []

    for key in meta_keys:
        value = getattr(meta, key)
        if value is not None or verbose:
            pairs.append(MetadataPair(key, value))

    return pairs


@click.command()
@click.argument("file", required=True, type=click.Path(exists=True))
@click.option("-v", "--verbose", is_flag=True, help="show empty fields")
@click.option("-i", "--image", "show_images", is_flag=True, help="show info for every image")
def main(file, verbose, show_images):
    if file is None:
        exit(1)

    mangofile = MangoFile.open(file)
    meta = mangofile.meta_data

    pairs = get_meta_pairs(meta, verbose)

    pairs.append(MetadataPair("images", mangofile.image_count))

    if show_images:
        pairs.append(MetadataPair("images", mangofile.image_count))
        for image in mangofile.images:
            meta = image.meta_data
            pairs.append(ImagePair("filename", meta.filename))
            pairs.append(ImagePair("mime", meta.mime))
            pairs.append(ImagePair("checksum", meta.checksum))

            if meta.compression is not None or verbose:
                pairs.append(ImagePair("compression", meta.compression))

            if meta.encryption is not None or verbose:
                pairs.append(ImagePair("encryption", meta.encryption))

            # add a blank line after every image
            pairs.append(None)

    for pair in pairs:
        if pair.__class__ == MetadataPair:
            click.echo("{:12}: {:<12}".format(pair.key, pair.value))
        elif pair.__class__ == ImagePair:
            click.echo("{:12}  {:12}: {:<12}".format("", pair.key, pair.value))
        elif pair is None:
            click.echo(" ")
