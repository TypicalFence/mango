#include <stdio.h>
#include "../libmango.h"

int main(void) {
	// This works fine, how it should
	MangoFile file = new_mango_file();
	mangofile_add_image_by_path(file, "test.jpg");
	MangoImage img = mangofile_get_image(file, 0);
	

	// This returns "oh no"
	//MangoImage img  = mangoimg_from_path("test.jpg");

	MangoImage meta = mangoimg_get_meta(img);
	printf(mangoimgmeta_checksum(meta));
	
	return 0;
}

