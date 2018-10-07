#include <stdio.h>
#include "../libmango.h"

int main(int argc, char *argv[]) {
    MangoImage img  = mangoimg_from_path(argv[1], NULL);

    ImageData data = mangoimg_get_image_data(img);

    uint8_t* it  = data.pointer;
    uint8_t* ite = it + data.length;
    printf("%ld\n", data.length);

    for(; it != ite; ++it) {
        printf("%i", (uint32_t)*it);
    }

    return 0;
}

