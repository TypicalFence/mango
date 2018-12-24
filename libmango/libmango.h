#include <stddef.h>
#include <stdint.h>

typedef void * MangoFile;
typedef void * MangoImage;
typedef void * MangoImageMeta;
typedef void * MangoMeta;

typedef struct ImageData {
    uint8_t* pointer;
    size_t length;
} ImageData;

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
 * \param file the file you want to free.
 */
extern void mangofile_free(MangoFile file);

/**
 * Adds a MangoImage to a MangoFile.
 *
 * \param file
 * \param image
 */
extern void mangofile_add_image(MangoFile file, MangoImage image);

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

// ----------------------------------------------------------------------------
// Mango Image
// ----------------------------------------------------------------------------
extern void mangoimg_free(MangoImage);
extern MangoImage mangoimg_from_path(char *, int *);
extern int mangoimg_compress(MangoImage, char *);
extern int mangoimg_uncompress(MangoImage);
extern MangoImageMeta mangoimg_get_meta(MangoImage);
extern ImageData mangoimg_get_image_data(MangoImage);
extern char * mangoimg_get_base64_image_data(MangoImage);
extern int mangoimg_encrypt(MangoImage, char *, char *);
extern int mangoimg_decrypt(MangoImage, char *);
extern int save(MangoImage, char *);

// ----------------------------------------------------------------------------
// Mango Image Meta
// ----------------------------------------------------------------------------
extern char * mangoimgmeta_compression(MangoImageMeta);
extern char * mangoimgmeta_encryption(MangoImageMeta);
extern char * mangoimgmeta_checksum(MangoImageMeta);
extern char * mangoimgmeta_mime(MangoImageMeta);
extern char * mangoimgmeta_filename(MangoImageMeta);
extern int  * mangoimgmeta_iv(MangoImageMeta);
extern int  * mangoimgmeta_iv_size(MangoImageMeta);

// ----------------------------------------------------------------------------
// Mango Meta
// ----------------------------------------------------------------------------
extern char * mangometa_get_title(MangoMeta);
extern void mangometa_set_title(MangoMeta, char *);

extern char * mangometa_get_author(MangoMeta);
extern void mangometa_set_author(MangoMeta, char *);

extern char * mangometa_get_publisher(MangoMeta);
extern void mangometa_set_publisher(MangoMeta, char *);

extern char * mangometa_get_source(MangoMeta);
extern void mangometa_set_source(MangoMeta, char *);

extern char * mangometa_get_translation(MangoMeta);
extern void mangometa_set_translation(MangoMeta, char *);

extern char * mangometa_get_language(MangoMeta);
extern void mangometa_set_language(MangoMeta, char *);

extern IntOption mangometa_get_volume(MangoMeta);
extern void mangometa_set_volume(MangoMeta, short *);

extern IntOption mangometa_get_chapter(MangoMeta);
extern void mangometa_set_chapter(MangoMeta, short *);

extern IntOption mangometa_get_year(MangoMeta);
extern void mangometa_set_year(MangoMeta, short *);

