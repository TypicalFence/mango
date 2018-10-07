#include <stdio.h>
#include "../libmango.h"

void main(void) {
    MangoImage img;

    img = mangoimg_from_path("test.jpg", NULL);
    mangoimg_encrypt(img, "AES128", "1234567812345678");
    
    int * iv = mangoimgmeta_iv(mangoimg_get_meta(img));
    
    int iv_size = mangoimgmeta_iv_size(mangoimg_get_meta(img));
    
    printf("[ ");
    
    for(int i = 0; i < iv_size; i++){
        printf( "%d", iv[i]);
        if(i != iv_size - 1) {
            printf( ", " );
        }
    }

    printf(" ]");
}
