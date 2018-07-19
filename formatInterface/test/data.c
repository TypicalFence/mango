#include <stdio.h>
#include "../libmango.h"

int main(void) {
	MangoImage img  = mangoimg_from_path("test.jpg");

	ImageData data = mangoimg_get_image_data(img);
	//for(int i = 0; i )
	//printf("%d", data.pointer[0]);
	
	uint8_t* it  = data.pointer;
	uint8_t* ite = it + data.length;

	for(; it != ite; ++it) {
		printf("%i", (uint32_t)*it);
	}
	return 0;
}

