#include <stdio.h>
#include "../libmango.h"

void main(void) {
    MangoFile file;
    MangoImage img;
    
    // create file
    file = new_mango_file();
    mangofile_add_image_by_path(file, "test.jpg");

    // get checksum
    img = mangofile_get_image(file, 0);
    char * checksum_before = mangoimgmeta_checksum(mangoimg_get_meta(img));

    // save
    mangofile_save(file, "testfile.mango");
    
    // print checksum
    printf("%s\n", checksum_before);
    
    // reset variables
    file = NULL;
    img = NULL;
    
    // open the created file
    file = mangofile_open("testfile.mango");
    
    // get checksum
    img = mangofile_get_image(file, 0);
    char * checksum_after = mangoimgmeta_checksum(mangoimg_get_meta(img));
    
    // print checksum
    printf("%s\n", checksum_after);

    remove("testfile.mango");
}
