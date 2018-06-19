from pymango import MangoImage

def test_open():
    img = MangoImage("test.jpg")


def test_checksum():
    import subprocess
    img_sum = MangoImage("test.jpg").meta_data.checksum
    sys_proc = subprocess.run(["sha256sum", "test.jpg"], stdout=subprocess.PIPE)
    sys_sum = sys_proc.stdout.decode("utf-8").split(" ")[0]
    

    print(img_sum)
    print(sys_sum)
    assert img_sum == sys_sum


