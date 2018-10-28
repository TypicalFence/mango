from mangofmt import MangoFile, EncryptionType, CompressionType, Language

def test_title():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.title == None
    meta.title = "test"
    assert meta.title == "test"

def test_author():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.author == None
    meta.author = "test"
    assert meta.author == "test"

def test_publisher():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.publisher == None
    meta.publisher = "test"
    assert meta.publisher == "test"

def test_source():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.source == None
    meta.source = "test"
    assert meta.source == "test"

def test_source():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.translation == None
    meta.translation = "test"
    assert meta.translation == "test"

def test_volume():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.volume == None
    meta.volume = 2
    assert meta.volume == 2

def test_chapter():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.chapter == None
    meta.chapter = 2
    assert meta.chapter == 2

def test_year():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.year == None
    meta.year = 2
    assert meta.year == 2

def test_language():
    mango = MangoFile()
    meta = mango.meta_data
    assert meta.language == None
    meta.language = Language.EN
    assert meta.language == Language.EN
