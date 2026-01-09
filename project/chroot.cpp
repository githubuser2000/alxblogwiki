#include <unistd.h>

void enter_chroot() {
    chdir("/var/www");
    chroot("/var/www");
    chdir("/");
}
