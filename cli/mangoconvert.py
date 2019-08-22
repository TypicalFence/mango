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
import os
import re
import random
import zipfile
import click

from copy import copy
from shutil import rmtree
from abc import ABC, abstractmethod
from mangofmt import MangoFile, CompressionType, MangoImage


def natural_sort(l):
    convert = lambda text: int(text) if text.isdigit() else text.lower()
    alphanum_key = lambda key: [convert(c) for c in re.split('([0-9]+)', key)]
    return sorted(l, key=alphanum_key)

# ----------------------------------------------------------------------------
# Interfaces
# ----------------------------------------------------------------------------

# The following 3 "interfaces" enable Formats to only be available in either
# "mangoconvert to"-call or "mangoconvert from"-call
# both could be implemented as well


class ConvertFrom(ABC):
    @abstractmethod
    def convert_from(self, path, out=None):
        pass


class ConvertTo(ABC):
    @abstractmethod
    def convert_to(self, mangofile, out=None):
        pass


class Format(ABC):
    def __init__(self):
        self._hash = str(random.getrandbits(128))

    @property
    @abstractmethod
    def name(self):
        pass

    @property
    def hash(self):
        return self._hash

    def register(self, cmd, from_group, to_group):
        cmd.name = self.name

        if issubclass(self.__class__, ConvertFrom):
            from_group.add_command(cmd, self.name)

        if issubclass(self.__class__, ConvertTo):
            to_group.add_command(cmd, self.name)

    @staticmethod
    def get_mango_file(path):
        files = natural_sort(os.listdir(path))
        mango_file = MangoFile()

        for file in files:
            # TODO check if add worked
            img = MangoImage.from_path(path + "/" + file)
            img.compress(CompressionType.GZIP)
            mango_file.add_image(img)

        return mango_file


# ----------------------------------------------------------------------------
# Formats
# ----------------------------------------------------------------------------
formats = []


class CbzFormat(Format, ConvertFrom, ConvertTo):
    @property
    def name(self):
        return "cbz"

    def convert_to(self, mangofile, out=None):
        os.mkdir("/tmp/" + self.hash)

        if out is None:
            outfile = "out.zip"
        else:
            outfile = out

        with zipfile.ZipFile(outfile, mode="w") as zip_file:
            for i in range(0, mangofile.image_count):
                img = mangofile.get_image(i)
                img_path = "/tmp/" + self.hash + "/" + os.path.basename(img.meta_data.filename)
                # save image and add it to the zip
                img.save(img_path)
                zip_file.write(img_path, os.path.basename(img.meta_data.filename))

        rmtree("/tmp/" + self.hash)

    def convert_from(self, path, out=None):
        os.mkdir("/tmp/" + self.hash)
        if out is None:
            outfile = "out.mango"
        else:
            outfile = out

        if zipfile.is_zipfile(path):
            zip = zipfile.ZipFile(path)
            names = zip.namelist()
            # remove files with file traversal
            names = list(filter(lambda x: not x.startswith("/") and "../" not in x, names))

            zip.extractall(path="/tmp/" + self.hash, members=names)
            zip.close()

            mango_file = self.get_mango_file("/tmp/" + self.hash)
            mango_file.save(outfile)

        rmtree("/tmp/" + self.hash)


formats.append(CbzFormat())

# ----------------------------------------------------------------------------
# Click Command
# ----------------------------------------------------------------------------


@click.group()
def mango_convert():
    pass


@mango_convert.group(name="to")
def to_group():
    pass


@mango_convert.group(name="from")
def from_group():
    pass


@click.command()
@click.argument("path")
@click.pass_context
def converter(ctx, path):
    # we can assume that the class that belongs to the command must exist
    # because this command gets registered by the class
    file_format = list(filter(lambda x: x.name == ctx.info_name, formats))[0]

    if ctx.parent.info_name == "from":
        file_format.convert_from(path)
    elif ctx.parent.info_name == "to":
        file_format.convert_to(MangoFile.open(path))


for file_format in formats:
    file_format.register(copy(converter), from_group, to_group)
