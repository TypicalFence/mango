#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <check.h>
#include <string.h>
#include "../libmango.h"
#include "test.h"

// TODO: rewrite this
// check if img is null
START_TEST(test_create) {
    MangoImage img;
    MangoImageMeta meta;
    
    int * fuck = NULL;

    img = mangoimg_from_path("test.jpg", fuck);

    ck_assert(fuck == 0);
    
    meta = mangoimg_get_meta(img);
    
    // strcmp returns 0 if the strings are equal
    ck_assert(strcmp(mangoimgmeta_filename(meta), "test.jpg") == 0);
}
END_TEST



START_TEST(test_decrypt) {
    MangoImage img;
    
    int * fuck = NULL;
    img = mangoimg_from_path("test.jpg", fuck);
    
    // encrypt
    mangoimg_encrypt(img, "AES128", "1234567812345678");
    char* enc_type_en = mangoimgmeta_encryption(mangoimg_get_meta(img));
    ck_assert(strcmp(enc_type_en, "AES128") == 0);

    
    // decrypt
    mangoimg_decrypt(img, "1234567812345678");
    char* enc_type_de = mangoimgmeta_encryption(mangoimg_get_meta(img));
    ck_assert(enc_type_de == NULL);
}
END_TEST

Suite * image_suite(void) {
    Suite *s;
    TCase *tc_core;

    s = suite_create("MangoImage");
    
    tc_core = tcase_create("Core");

    // Core
    tcase_add_test(tc_core, test_create);
    tcase_add_test(tc_core, test_decrypt);

    // add cases to suites
    suite_add_tcase(s, tc_core);

    return s;
}

Suite * make_image_suite() {
    return image_suite();
}
