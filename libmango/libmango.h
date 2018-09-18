#include <stdint.h>

typedef void * MangoFile;
typedef void * MangoImage;
typedef void * MangoImageMeta;
typedef void * MangoMeta;

typedef struct ImageData {
    uint8_t* pointer;
    size_t length;
} ImageData;

//---------------------------
// Mango File
// --------------------------
extern MangoFile new_mango_file(); // TODO rename to mangofile_new
extern void mangofile_new(MangoFile);
extern void mangofile_add_image(MangoFile, MangoImage);
extern void mangofile_add_image_by_path(MangoFile, char *);
extern MangoImage mangofile_get_image(MangoFile, int);
extern MangoImage mangofile_set_image(MangoFile, int);
extern int mangofile_get_image_count(MangoFile);
extern MangoMeta mangofile_get_meta(MangoFile);

// save
extern void mangofile_save(MangoFile, char *);
extern void mangofile_save_cbor(MangoFile, char *);
extern void mangofile_save_bson(MangoFile, char *);
extern void mangofile_save_json(MangoFile, char *);
// open
extern MangoFile mangofile_open(char *);

// --------------------------
// Mango Image
// --------------------------
extern void mangoimg_free(MangoImage);
extern MangoImage mangoimg_from_path(char *);
extern int mangoimg_compress(MangoImage, char *);
extern int mangoimg_uncompress(MangoImage);
extern MangoImageMeta mangoimg_get_meta(MangoImage);
extern ImageData mangoimg_get_image_data(MangoImage);
extern char * mangoimg_get_base64_image_data(MangoImage);
// TODO
// extern int mangoimg_encrypt(MangoImage, char *);
// extern int mangoimg_decryopt(MangoImage);
// extern void save(char *);


// --------------------------
// Mango Image Meta
// --------------------------
extern char * mangoimgmeta_compression(MangoImageMeta);
extern char * mangoimgmeta_encryption(MangoImageMeta);
extern char * mangoimgmeta_checksum(MangoImageMeta);
extern char * mangoimgmeta_mime(MangoImageMeta);
extern char * mangoimgmeta_filename(MangoImageMeta);
// extern int  * mangoimgmeta_iv(MangoImageMeta);)

// --------------------------
// Mango Meta
// --------------------------
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

// TODO
// lang
// volume
// chapter
// year
