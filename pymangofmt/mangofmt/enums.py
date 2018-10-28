from enum import Enum

class CompressionType(Enum):
    GZIP = "GZIP"


class EncryptionType(Enum):
    AES128 = "AES128"
    AES256 = "AES256"

class Language(Enum):
    EN = "EN"
    JP = "JP"
    DE = "DE"
    FR = "FR"
    IT = "IT"
    CN = "CN"
    ES = "ES"

