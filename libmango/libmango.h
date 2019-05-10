#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Repesents a MangoFile sturct from the mangofmt rust library.
 *
 * A MangoFile contains an instance of ManoMeta and can conatain multiple MangoImages.
 */
typedef void * MangoFile;

/**
 * Repesents a MangoImage sturct from the mangofmt rust library.
 *
 * A MangoImage conatains an instance of MangoImageMeta.
 */
typedef void * MangoImage;

/**
 * Repesents a MangoImageMeta sturct from the mangofmt rust library.
 *
 * A MangoImageMeta instance always belongs to a MangoImage.
 */
typedef void * MangoImageMeta;

/**
 * Repesents a MangoMeta sturct from the mangofmt rust library.
 *
 * A MangoMeta instance always belongs to a MangoFile.
 */
typedef void * MangoMeta;

typedef struct ImageData {
    uint8_t* pointer;
    size_t length;
} ImageData;

/**
 * This struct maps to a rust Option containing an int.
 */
typedef struct IntOption {
    int value;
    int present;
} IntOption;

/**
 * Checks if support for encryption was compiled in.
 *
 * \param enc_type the encryption type you want to check.
 *
 * \returns 0 or 1 depending on if it is supported or not.
 */
extern int mango_encryption_is_supported(char *enc_type);

/**
 * Checks if support for compression was compiled in.
 *
 * \param comp_type the compression type you want to check.
 *
 * \returns 0 or 1 depending on if it is supported or not.
 */
extern int mango_compression_is_supported(char *comp_type);


//-------------------------------------------------------------------------------------------------
// Mango File
// ------------------------------------------------------------------------------------------------

/**
 * Creates a new MangoFile.
 *
 * \returns the newly created MangoFile.
 */
extern MangoFile mangofile_new();

/**
 * Frees the memory of a MangoFile.
 *
 * this will also free all child elements, like MangoImage, and MangoMeta.
 * It will also free the MangoImageMeta children of the MangoImage instances.
 *
 * it is important that if you have an instance of the child element, 
 * that you must not free the child before the parent 
 * and when you free the parent all child references will be gone too. 
 *
 * so keep that in mind when writting your code.
 * 
 * \param file the file you want to free.
 */
extern void mangofile_free(MangoFile file);

/**
 * Adds a MangoImage to a MangoFile.
 *
 * \param file
 * \param image
 *
 * * \returns an error code:
 * - 0 ok
 * - -42 either of the pointers passed are null
 */
extern int mangofile_add_image(MangoFile file, MangoImage image);

/**
 * Adds a image to a MangoFile.
 *
 * \param file
 * \param path
 *
 * \returns an error code:
 * - 0 ok
 * - 1 permission denied
 * - -1 unknown error
 * - -42  MangoFile is null
 */
extern int mangofile_add_image_by_path(MangoFile file , char *path);

/**
 * Gets a MangoImage from the MangoFile by ID.
 *
 * \param file
 * \param id
 *
 * \returns the MangoFile requested, might be NULL
 */
extern MangoImage mangofile_get_image(MangoFile file, int id);

/**
 * Sets a MangoImage in a MangoFile with a specific index.
 * 
 * \note you can't add a new image to a MangoFile with this function.
 *
 * \param file
 * \param image
 * \param id must be 0 or less that the image count of *file*
 *
 * \returns 0 or 1 depending on if it worked.
 */
extern int  mangofile_set_image(MangoFile file, MangoImage image, int index);

/**
 * Removes an image inside a MangoFile and shifts all images after it to the left.
 *
 * \param file
 * \param id must be an index that exists in the *file*
 *
 * \returns 0 or 1 depending on if it worked.
 */
extern int  mangofile_remove_image(MangoFile file, int id);

/**
 * Gets the count of all Images of a MangoFile.
 *
 * \param file
 *
 * \returns image count
 */
extern int mangofile_get_image_count(MangoFile file);

/**
 * Gets the metadata of a MangoFile.
 *
 * \param file
 *
 * \returns MangoMeta 
 */
extern MangoMeta mangofile_get_meta(MangoFile file);

/**
 * Saves a MangoFile to the file system.
 *
 * This Function uses the default format "cbor".
 *
 * If you want to use one of the other supported
 * formats specifically  please use one of the following functions:
 *
 * - \link mangofile_save_cbor \endlink
 * - \link mangofile_save_json \endlink
 * - \link mangofile_save_bson \endlink

 *
 * All formats should use the file ending ".mango"
 * and can be opened with \link mangofile_open \endlink.
 * 
 * \param file the file to save
 * \param path the path where the file will get saved to
 *
 * \returns an error code containing if the file could have been saved.
 *
 * \return The error codes mean the following:
 * - 0 everything went ok
 * - 1 encode error
 * - 2 write error
 * - 3 permission error
 * - -1 input parameters weren't okay
 */
extern int mangofile_save(MangoFile file, char *path);

/**
 * Saves a MangoFile in the cbor format to the file system.
 *
 * \note his is currently the default format, because it has the
 * lowest overhead!
 *
 * Following formats are also available:
 * - \link mangofile_save_json \endlink
 * - \link mangofile_save_bson \endlink
 *
 * You should save your file with ending ".mango".
 *
 * \param file the file to save
 * \param path the path where the file will get saved to
 *
 * \returns an error code containing if the file could have been saved.
 *
 * \return The error codes mean the following:
 * - 0 everything went ok
 * - 1 encode error
 * - 2 write error
 * - 3 permission error
 * - -1 input parameters weren't okay
 */
extern int mangofile_save_cbor(MangoFile file, char *path);

/**
 * Saves a MangoFile in the bson format to the file system.
 *
 * Following formats are also available:
 * - \link mangofile_save_json \endlink
 * - \link mangofile_save_cbor \endlink
 *
 * You should save your file with ending ".mango".
 * 
 * \param file the file to save
 * \param path the path where the file will get saved to
 *
 * \returns an error code containing if the file could have been saved.
 *
 * \return The error codes mean the following:
 * - 0 everything went ok
 * - 1 encode error
 * - 2 write error
 * - 3 permission error
 * - -1 input parameters weren't okay
 */
extern int mangofile_save_bson(MangoFile file, char *path);

/**
 * Saves a MangoFile in the json format to the file system.
 *
 * \warning You shouldn't really use this one,
 * the overhead that gets generated by the base64 encoding is way too big.
 * This is more meant for debuging purposes.
 *
 * \note There are currently no plans to depreciate the json format.
 *
 * Following formats are also available:
 * - \link mangofile_save_bson \endlink
 * - \link mangofile_save_cbor \endlink
 *
 * You should save your file with ending ".mango".
 *
 * \param file the file to save
 * \param path the path where the file will get saved to
 *
 * \returns an error code containing if the file could have been saved.
 *
 * \return The error codes mean the following:
 * - 0 everything went ok
 * - 1 encode error
 * - 2 write error
 * - 3 permission error
 * - -1 input parameters weren't okay
 */
extern int mangofile_save_json(MangoFile file, char *path);

/**
 * opens a MangoFile from the file system.
 *
 * \note The internal format of the MangoFile does not matter.
 * This function will detect the format and open it accordingly.
 *
 * \param path the path to the file
 * \param error 
 * \parablock
 * The Error code will be set in the variable passed in:
 * - 0 everything went ok
 * - 1 decode error
 * - 2 read error
 * - 3 permission error
 * - -1 input parameters weren't okay 
 * \endparablock
 *
 * \returns the opened MangoFile
 */
extern MangoFile mangofile_open(char * path, int * error);

// -------------------------------------------------------------------------------------------------
// Mango Image
// ------------------------------------------------------------------------------------------------

/**
 * Frees the memory of a MangoImage.
 * 
 * \param img the image you want to free.
 */
extern void mangoimg_free(MangoImage img);

/**
 * Creates a new MangoImage Struct out of a file from the harddrive.
 * 
 * \info currently only jpg & png are supported formats, more are planed in the future.
 *
 * \param path the path to the image file, you should prefer full paths, tho relative paths should work too
 * \param error 
 * \parablock
 * The Error code, it will be set in the variable passed in:
 * - 0 everything went ok
 * - 1 file not found
 * - -1 input parameters (probably) weren't okay or anything that should not have wen't wrong, went wrong.
 * \endparablock

 * \returns instance of MangoImage, containg the data of the image file.
 */
extern MangoImage mangoimg_from_path(char * path, int * error);

/**
 * Compresses the data of an MangoImage
 * 
 * \info Currently only GZIP is supported.
 * \info You can use \link mango_compression_is_supported \endlink to check if support for a certain type was compiled in.
 *
 * \param image
 * \param type The type of compresseion you want to apply.
 * 
 * \todo the return type doesn't appear to be an actual boolean.
 *
 * \returns 1 if everything went okay and 2 if some error occurred  
 */
extern int mangoimg_compress(MangoImage, char * type);

/**
 * Uncompresses the data of an compressed MangoImage.
 *
 * You don't need to specify a compression type, it is stored in the metadata.
 *
 * \todo the return type doesn't appear to be an actual boolean
 * \todo maybe it should also return something if the image wasn't compressed at all
 *
 * \param image
 * 
 * \returns 1 if everything went okay and 2 if some error occurred  
 */
extern int mangoimg_uncompress(MangoImage image);

/**
 * Gets the metadata of a MangImage.
 *
 * \param image
 *
 * \returns ManoImageMeta
 */
extern MangoImageMeta mangoimg_get_meta(MangoImage image);

/**
 * Returns the actual data of the imagefile stored inside of the MangoImage.
 * 
 * The ImageData struct has pointer to the data and the size of the data (in bytes), 
 * use those to read all of those bytes.
 * 
 * \param image
 * 
 * \returns ImageData struct
 */
extern ImageData mangoimg_get_image_data(MangoImage image);

/**
 * Returns the actual data of the image file stored inside of the MangoImage, encoded as an base64 string.
 * 
 * You aren't supposed to call this, it creates an overhead, 
 * this mainly exists because I couldn't get python to do what I want.
 *
 * Use mangoimg_get_image_data() instead!
 *
 * \returns the image data encoded as base64
 */
extern char * mangoimg_get_base64_image_data(MangoImage image);

/**
 * Encrypt the data of an MangoImage
 * 
 * \info Currently only "AES128" & "AES256" are supported
 * \info You can use \link mango_encryption_is_supported \endlink to check if support for a certain type was compiled in.
 *
 * \param image
 * \param type The type of encryption you want to apply.
 * \param password The password you want to use securiong the encryption.
 * 
 * \todo the return type doesn't appear to be an actual boolean.
 *
 * \returns 1 if everything went okay and 2 if some error occurred  
 */
extern int mangoimg_encrypt(MangoImage image, char * type, char * password);

/**
 * Decrypt the data of an compressed MangoImage.
 *
 * You don't need to specify a type, it's stored in the Metadata
 *
 * \todo the return type doesn't appear to be an actual boolean
 *
 * \param image
 * \param password
 * 
 * \returns 1 if everything went okay and 2 if some error occurred  
 */
extern int mangoimg_decrypt(MangoImage image, char * password);

/**
 * Saves the MangoImage to a file.
 *
 * \param image the image to save
 * \param filename the path/filename of the file to be created
 *
 * \returns The return codes mean:
 * - 0 everything OK!
 * - 1 permission denied!
 * - -1 we just don't know!     
 *
 */
extern int mangoimg_save(MangoImage, char *);

// ------------------------------------------------------------------------------------------------
// Mango Image Meta
// ------------------------------------------------------------------------------------------------

/**
 * Gets the compression type from a MangoImageMeta.
 *
 * \returns the compression type of a MangoImage.
 */
extern char * mangoimgmeta_compression(MangoImageMeta);

/**
 * Gets the encryption type from a MangoImageMeta.
 *
 * \returns the encryption type of a MangoImage.
 */
extern char * mangoimgmeta_encryption(MangoImageMeta);

/**
 * Gets the checksum from a MangoImageMeta.
 *
 * \returns the checksum of a MangoImage.
 */
extern char * mangoimgmeta_checksum(MangoImageMeta);

/**
 * Gets the mime type  from a MangoImageMeta.
 *
 * \returns the mime type of a MangoImage.
 */
extern char * mangoimgmeta_mime(MangoImageMeta);

/**
 * Gets the filename from a MangoImageMeta.
 *
 * \returns the filename of a MangoImage.
 */
extern char * mangoimgmeta_filename(MangoImageMeta);

/**
 * Gets the iv from a MangoImageMeta.
 *
 * \returns the iv of a MangoImage.
 */
extern int  * mangoimgmeta_iv(MangoImageMeta);

/**
 * Gets the iv size from a MangoImageMeta.
 *
 * \returns the size of the  iv of a MangoImage.
 */
extern int  * mangoimgmeta_iv_size(MangoImageMeta);

// ------------------------------------------------------------------------------------------------
// Mango Meta
// ------------------------------------------------------------------------------------------------

/**
 * Gets the title from a MangoMeta.
 *
 * \returns the title of a MangoFile.
 */
extern char * mangometa_get_title(MangoMeta meta);

/**
 * Sets the title of a MangoMeta.
 */
extern void mangometa_set_title(MangoMeta meta, char *value);

/**
 * Gets the author from a MangoMeta.
 *
 * \returns the author of a MangoFile.
 */
extern char * mangometa_get_author(MangoMeta meta);

/**
 * Sets the author of a MangoMeta.
 */
extern void mangometa_set_author(MangoMeta, char *);


/**
 * Gets the publisher from a MangoMeta.
 *
 * \returns the publisher of a MangoFile.
 */
extern char * mangometa_get_publisher(MangoMeta meta);

/**
 * Sets the publisher of a MangoMeta.
 */
extern void mangometa_set_publisher(MangoMeta, char *);


/**
 * Gets the source from a MangoMeta.
 *
 * \returns the source of a MangoFile.
 */
extern char * mangometa_get_source(MangoMeta meta);

/**
 * Sets the source of a MangoMeta.
 */
extern void mangometa_set_source(MangoMeta, char *);


/**
 * Gets the translation from a MangoMeta.
 *
 * \returns the translation of a MangoFile.
 */
extern char * mangometa_get_translation(MangoMeta meta);

/**
 * Sets the translation of a MangoMeta.
 */
extern void mangometa_set_translation(MangoMeta, char *);


/**
 * Gets the language from a MangoMeta.
 *
 * \returns the language of a MangoFile in a short form of 2-3 uppercase characters.
 */
extern char * mangometa_get_language(MangoMeta meta);

/**
 * Sets the language of a MangoMeta.
 *
 * \param meta
 * \param value must be a valid Language, please check the mangofmt docs for the Language Enum.
 */
extern void mangometa_set_language(MangoMeta meta, char *value);


/**
 * Gets the volume from a MangoMeta.
 *
 * \returns what volume a MangoFile contains.
 */
extern IntOption mangometa_get_volume(MangoMeta meta);

/**
 * Sets the volume of a MangoMeta.
 *
 * \param meta
 * \param value can be NULL
 */
extern void mangometa_set_volume(MangoMeta meta, short *value);


/**
 * Gets the chapter from a MangoMeta.
 *
 * \returns what chapter a MangoFile contains.
 */
extern IntOption mangometa_get_chapter(MangoMeta meta);

/**
 * Sets the chapter of a MangoMeta.
 *
 * \param meta
 * \param value can be NULL
 */
extern void mangometa_set_chapter(MangoMeta meta, short *value);


/**
 * Gets the year from a MangoMeta.
 *
 * \returns what year a MangoFile is from.
 */
extern IntOption mangometa_get_year(MangoMeta meta);

/**
 * Sets the year of a MangoMeta.
 *
 * \param meta
 * \param value can be NULL
 */
extern void mangometa_set_year(MangoMeta meta, short *value);


#ifdef __cplusplus
}
#endif

