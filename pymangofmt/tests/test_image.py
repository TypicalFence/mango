import os
import pytest
import subprocess
from mangofmt import MangoImage, EncryptionType, CompressionType

def test_open():
    img = MangoImage.from_path("test.jpg")

def test_filename():
    img = MangoImage.from_path("test.jpg")
    assert img.meta_data.filename == "test.jpg"

def test_mime():
    img = MangoImage.from_path("test.jpg")
    assert img.meta_data.mime == "JPEG"


def test_checksum():
    import subprocess
    img = MangoImage.from_path("test.jpg")
    meta = img.meta_data
    img_sum = meta.checksum
    sys_proc = subprocess.run(["sha256sum", "test.jpg"], stdout=subprocess.PIPE)
    sys_sum = sys_proc.stdout.decode("utf-8").split(" ")[0]

    print(img_sum)
    print(sys_sum)
    assert img_sum == sys_sum

def test_checksum_onelinner():
    import subprocess
    img_sum = MangoImage.from_path("test.jpg").meta_data.checksum
    sys_proc = subprocess.run(["sha256sum", "test.jpg"], stdout=subprocess.PIPE)
    sys_sum = sys_proc.stdout.decode("utf-8").split(" ")[0]

    print(img_sum)
    print(sys_sum)
    assert img_sum == sys_sum

def test_encryption_none():
    img_enc = MangoImage.from_path("test.jpg").meta_data.encryption
    assert img_enc is None

def test_compress():
    img = MangoImage.from_path("test.jpg")
    img_data = img.image_data
    img.compress(CompressionType.GZIP)
    assert not img_data == img.image_data

def test_uncompress():
    img = MangoImage.from_path("test.jpg")
    img_data = img.image_data
    img.compress(CompressionType.GZIP)
    img.uncompress()
    assert img_data == img.image_data

def test_encrypt():
    img = MangoImage.from_path("test.jpg")
    img_data = img.image_data
    img.encrypt(EncryptionType.AES128, "1234567812345678")
    assert not img_data == img.image_data

def test_decrypt():
    img = MangoImage.from_path("test.jpg")
    img_data = img.image_data

    enc = img.encrypt(EncryptionType.AES128, "1234567812345678")
    assert enc == True

    dec = img.decrypt("1234567812345678")
    assert dec == True

    assert img_data == img.image_data

def test_save():
    img = MangoImage.from_path("test.jpg")
    img.save("save_test.jpg")

@pytest.mark.skipif(os.geteuid() == 0, reason="can't be tested as root")
def test_save_permission():
    img = MangoImage.from_path("test.jpg")

    with pytest.raises(PermissionError):
        img.save("/test.jpg")

def test_iv():
    img = MangoImage.from_path("test.jpg")
    img.encrypt(EncryptionType.AES128, "lol")
    meta = img.meta_data
    iv = meta.iv

    # is a list of random ints
    assert isinstance(iv, list)
    assert len(iv) > 0
    assert isinstance(iv[0], int)
