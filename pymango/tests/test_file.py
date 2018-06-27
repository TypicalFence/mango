from pymango import MangoImage, EncryptionType, CompressionType, MangoFile


def test_meta_is_none(): 
    mango_file = MangoFile()

    assert mango_file.meta.author is  None

def test_set_meta():
    mango_file = MangoFile()
    # a = mango_file.meta.author
    print(mango_file.meta)
    mango_file.meta.author = "someone"
    print(mango_file.meta.author)
    assert mango_file.meta.author == "someone"


def test_file():
    m = MangoFile()
    m.add_image_by_path("test.jpg")
    img = m.get_image(0)
    img.compress(CompressionType.GZIP)
    sum1 = img.meta_data.checksum
    sum2 = m.get_image(0).meta_data.checksum
    print(sum1)
    print(sum2)
    print(img.meta_data.checksum)
     
test_file()
#test_set_meta()
