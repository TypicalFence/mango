from pymango import MangoImage, EncryptionType, CompressionType

def test_open():
    img = MangoImage.from_path("test.jpg")

def test_filename():
    img = MangoImage.from_path("test.jpg")
    assert img.meta_data.filename == "test.jpg"

def test_checksum():
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
