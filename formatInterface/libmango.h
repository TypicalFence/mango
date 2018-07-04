typedef void * MangoFile;
typedef void * MangoImage;
typedef void * MangoMeta;

// Mango File
extern MangoFile new_mango_file();
extern void mangofile_add_image(MangoFile, char *);
extern MangoImage mangofile_get_image(MangoFile, int);
extern MangoMeta mangofile_get_meta(MangoFile);
// Mango Image
extern int mangoimg_compress(MangoImage);
extern int mangoimg_is_compressed(MangoImage);

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
