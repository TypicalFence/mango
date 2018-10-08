#include <stdio.h>
#include "../libmango.h"

int main(int argc, char *argv[]) {
    int error = -10;
    MangoImage img = mangoimg_from_path(argv[1], &error);

    if (error == 0) {
        MangoImageMeta meta = mangoimg_get_meta(img);

        printf("%s\n", mangoimgmeta_checksum(meta));
    }

    return error;
}

