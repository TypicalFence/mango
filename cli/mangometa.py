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
import types
import click

from copy import copy
from mangofmt import MangoFile, Language

# this list contains all the keys of the MangoMeta object
meta_keys = []

string_keys = ["title", "author", "publisher", "source", "translation"]
num_keys = ["volume", "chapter", "year"]

meta_keys.extend(string_keys)
meta_keys.append("language")
meta_keys.extend(num_keys)


def validate_key(meta_key, value):
    """Validates a input value matched to the key that it should be written to
        Args:
            meta_key (str): The key it should be written to
            value (var): The Value to be written
       
        Returns:
            bool: True if valid, False otherwise. 
    """
    string_keys = ["title", "author", "publisher", "source", "translation"]   
    num_keys = ["volume", "chapter", "year"]

    if meta_key in string_keys:
        return validate_str(value)
    elif meta_key == "language":
        return validate_lang(value)
    elif meta_key in num_keys:
        return validate_num(value)
    else:
        return False


validate_str = lambda x: isinstance(x, str)


def validate_num(value):
    try:
        int(value)
        return True
    except Exception:
        return False


def validate_lang(value):
    try:
        Language(value.upper())
        return True
    except Exception:
        return False


@click.group()
@click.argument("file", nargs=1, required=True)
@click.pass_context
def main(ctx, file):
    """Get and set values from the metadata of a mango file."""
    ctx.ensure_object(types.SimpleNamespace)
    ctx.obj.file = file


# the element function will be reused multiple times for each meta_key
@click.command()
# the value argument may have to be a string or an int, 
# depending on the meta_key, therefor we'll just expect a string
@click.argument("value", nargs=1, required=False)
@click.pass_context
def element(ctx, value):
    """
    Get & set values from the {0} field


    Calling it without passing in a value will print the current value.
    If a value is given, it will be set inside the file as the new value.
    """
    mangofile = MangoFile.open(ctx.obj.file)
    meta = mangofile.meta_data
    
    if value is None:
        current_value = getattr(meta, ctx.info_name)
        if current_value is not None:
            click.echo(current_value)
        else:
            exit(1)
    else:
        if validate_key(ctx.info_name, value):
            if ctx.info_name in num_keys:
                value = int(value)
            elif ctx.info_name == "language":
                value = Language(value.upper())
            
            setattr(meta, ctx.info_name, value)
            mangofile.save(ctx.obj.file)
        else:
            click.echo("Can't set {0}: Invalid value".format(ctx.info_name))
            exit(1)


# dynamically create the element command for all needed keys
for key in meta_keys:
    # make a copy of element
    # if you don't all command will have the same help string
    # because there will actually only be one instance of the function element 
    # inside the main command group
    main.add_command(copy(element), key)
    
    # set the name of the key in the help text
    cmd = main.get_command(click.Context(main), key)
    cmd.help = cmd.help.format(key)

