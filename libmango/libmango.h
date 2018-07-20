#include <stdint.h>

typedef void * MangoFile;
typedef void * MangoImage;
typedef void * MangoImageMeta;
typedef void * MangoMeta;

typedef struct ImageData {
    uint8_t* pointer;
    size_t length;
} ImageData;

// Mango File
extern MangoFile new_mango_file();
extern void mangofile_add_image_by_path(MangoFile, char *);
extern MangoImage mangofile_get_image(MangoFile, int);
extern MangoMeta mangofile_get_meta(MangoFile);

// Mango Image
extern MangoImage mangoimg_from_path(char *);
extern int mangoimg_compress(MangoImage, char *);
extern int mangoimg_uncompress(MangoImage);
extern MangoImageMeta mangoimg_get_meta(MangoImage);
extern ImageData mangoimg_get_image_data(MangoImage);
extern char * mangoimg_get_base64_image_data(MangoImage);

// Mango Image Meta
extern char * mangoimgmeta_compression(MangoImageMeta);
extern char * mangoimgmeta_encryption(MangoImageMeta);
extern char * mangoimgmeta_checksum(MangoImageMeta);

// Mango Meta
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
