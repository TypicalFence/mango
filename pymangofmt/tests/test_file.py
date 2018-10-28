import os
from mangofmt import MangoImage, EncryptionType, CompressionType, MangoFile


def test_meta_is_none():
    mango_file = MangoFile()

    assert mango_file.meta_data.author is  None

def test_set_meta():
    mango_file = MangoFile()
    mango_file.meta_data.author = "someone"
    assert mango_file.meta_data.author == "someone"


def test_save_meta():
    mango_file = MangoFile()
    mango_file.meta_data.author = "someone"
    mango_file.save_json("save_meta.mango")

    del mango_file

    mango_file = MangoFile.open("save_meta.mango")

    assert mango_file.meta_data.author == "someone"
    os.remove("save_meta.mango")

def test_save():
    file = MangoFile()
    meta = file.meta_data
    meta.author = "someone"

    file.add_image_by_path("test.jpg")

    file.save("file_save.mango")

    assert os.path.isfile("file_save.mango")
    os.remove("file_save.mango")

def test_save_cbor():
    file = MangoFile()
    meta = file.meta_data
    meta.author = "someone"

    file.add_image_by_path("test.jpg")

    file.save("file_save_cbor.mango")

    assert os.path.isfile("file_save_cbor.mango")
    os.remove("file_save_cbor.mango")

def test_save_json():
    file = MangoFile()
    meta = file.meta_data
    meta.author = "someone"

    file.add_image_by_path("test.jpg")

    file.save("file_save_json.mango")

    assert os.path.isfile("file_save_json.mango")
    os.remove("file_save_json.mango")

def test_save_bson():
    file = MangoFile()
    meta = file.meta_data
    meta.author = "someone"

    file.add_image_by_path("test.jpg")

    file.save("file_save_bson.mango")

    assert os.path.isfile("file_save_bson.mango")
    os.remove("file_save_bson.mango")

def test_open():
    file = MangoFile()
    meta = file.meta_data
    meta.author = "someone"

    file.add_image_by_path("test.jpg")

    file.save("file_open.mango")

    ofile = MangoFile.open("file_open.mango")
    assert ofile.meta_data.author == file.meta_data.author
    os.remove("file_open.mango")
