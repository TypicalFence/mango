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
import click
from mangofmt import MangoFile
from mangofmt.error import WriteError


def get_env_vars(ctx, args, incomplete):
    return [k for k in os.environ.keys() if incomplete in k]


@click.command()
@click.argument("filename")
@click.option("--json", is_flag=True, help="use json as the format")
@click.option("--bson", is_flag=True, help="use bson as the format")
@click.option("--cbor", is_flag=True, help="use cbor as the format (default)")
def main(filename, json, bson, cbor):
    """
    Creates a new & empty Mangofile.

    It will not overwrite anyfiles! 
    """
    if os.path.isfile(filename):
        click.echo("File already exists")
        exit(1)

    file = MangoFile()

    try:
        if json:
            file.save_json(filename)
        elif bson:
            file.save_bson(filename)
        elif cbor:
            file.save_cbor(filename)
        else:
            file.save(filename)

        exit(0)
    except WriteError:
        click.echo("WriteError: could not create " + filename)
        exit(3)
    except PermissionError:
        click.echo("Permission denied: could not create " + filename)
        exit(4)
