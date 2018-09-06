#include <stdlib.h>
#include <check.h>
#include "test.h"

int main(void) {
    int number_failed;
    SRunner *sr;

    sr = srunner_create(make_file_suite());

    srunner_run_all(sr, CK_VERBOSE);
    number_failed = srunner_ntests_failed(sr);
    srunner_free(sr);
    return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}

