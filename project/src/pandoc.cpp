#include "pandoc.h"
#include <unistd.h>
#include <sys/wait.h>
#include <iostream>

bool render_with_pandoc_safe(const std::filesystem::path& file) {
    int fd[2];
    if (pipe(fd) != 0) return false;

    pid_t pid = fork();
    if (pid == 0) {
        dup2(fd[1], STDOUT_FILENO);
        close(fd[0]);

        const char* from =
            file.extension() == ".org" ? "org" : "markdown";

        execlp("pandoc", "pandoc",
            "--standalone",
            "--from", from,
            "--to", "html5",
            file.c_str(),
            nullptr);

        _exit(1);
    }

    close(fd[1]);
    char buf[4096];
    ssize_t n;
    while ((n = read(fd[0], buf, sizeof(buf))) > 0)
        std::cout.write(buf, n);

    close(fd[0]);
    int status;
    waitpid(pid, &status, 0);
    return WIFEXITED(status) && WEXITSTATUS(status) == 0;
}
