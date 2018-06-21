#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <check.h>

extern void * new_mango_file(char *);
extern char * mangofile_get_name(void *);

START_TEST(test_create) {
  void *file;
  char *name;
  file = new_mango_file("yay");
  name = mangofile_get_name(file);
  ck_assert_str_eq(name, "yay");
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
