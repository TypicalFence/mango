#include <stdio.h>
#include "../libmango.h"

int main(int argc, char *argv[]) {
    MangoImage img = mangoimg_from_path(argv[1], NULL);
    MangoImageMeta meta = mangoimg_get_meta(img);

    printf("%s\n", mangoimgmeta_checksum(meta));

    return 0;
}

