#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <check.h>

extern void * new_mango_file();
extern void mangofile_add_image(void *, char *);
extern void * mangofile_get_image(void *, int);
extern int8_t mangoimg_compress(void *);
extern int8_t mangoimg_is_compressed(void *);

START_TEST(test_create) {
  void *file;
  void *img;
  file = new_mango_file();
  mangofile_add_image(file, "test.jpg");

  img = mangofile_get_image(file, 0);
  mangoimg_compress(img);

  img = mangofile_get_image(file, 0);
  int8_t yay = mangoimg_is_compressed(img);
  ck_assert(yay == 1);
}
END_TEST

Suite * mango_suite(void) {
	Suite *s;
	TCase *tc_core;
	

	s = suite_create("Mango");
	tc_core = tcase_create("Core");

	tcase_add_test(tc_core, test_create);
	suite_add_tcase(s, tc_core);

	return s;
}

int main(void) {
	int number_failed;
	Suite *s;
        SRunner *sr;

	s = mango_suite();
	sr = srunner_create(s);

	srunner_run_all(sr, CK_NORMAL);
	number_failed = srunner_ntests_failed(sr);
	srunner_free(sr);
	return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
