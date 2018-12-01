from enum import Enum
from mangofmt.c import libmango

class CompressionType(Enum):
    GZIP = "GZIP"

    def is_supported(self):
        print(self)
        return libmango.mango_compression_is_supported(self.value.encode("utf-8"))

class EncryptionType(Enum):
    AES128 = "AES128"
    AES256 = "AES256"

    def is_supported(self):
        return libmango.mango_encryption_is_supported(self.value.encode("utf-8"))

class Language(Enum):
    EN = "EN"
    JP = "JP"
    DE = "DE"
    FR = "FR"
    IT = "IT"
    CN = "CN"
    ES = "ES"

