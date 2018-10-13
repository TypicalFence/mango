class MangoFileError(Exception):
   pass

class EncodeError(MangoFileError):
   pass
   
class DecodeError(MangoFileError):
    pass
   
class ReadError(MangoFileError):
    pass
    
class WriteError(MangoFileError):
    pass
