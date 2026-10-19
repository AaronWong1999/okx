#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h>

int main(int argc, char **argv) {
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--list-modules") == 0) {
            printf("java.base@21.0.2\njdk.compiler@21.0.2\njava.xml@21.0.2\njava.logging@21.0.2\n");
            fflush(stdout);
            return 0;
        }
    }
    int in_pipe[2], out_pipe[2];
    pipe(in_pipe);
    pipe(out_pipe);
    pid_t pid = fork();
    if (pid == 0) {
        dup2(in_pipe[0], STDIN_FILENO);
        dup2(out_pipe[1], STDOUT_FILENO);
        close(in_pipe[0]); close(in_pipe[1]);
        close(out_pipe[0]); close(out_pipe[1]);
        execl("/usr/bin/env", "env", "python3", "/Users/aaron/Desktop/aaron/okx/tools/rea_android_engine/rea_jadx_mcp_bridge.py", NULL);
        _exit(1);
    }
    close(in_pipe[0]);
    close(out_pipe[1]);
    // Forward stdin/stdout between REA and python child while keeping this Mach-O process as the owned leader
    pid_t reader = fork();
    if (reader == 0) {
        close(out_pipe[0]);
        char buf[4096];
        ssize_t n;
        while ((n = read(STDIN_FILENO, buf, sizeof(buf))) > 0) {
            write(in_pipe[1], buf, n);
        }
        close(in_pipe[1]);
        _exit(0);
    }
    close(in_pipe[1]);
    char buf[4096];
    ssize_t n;
    while ((n = read(out_pipe[0], buf, sizeof(buf))) > 0) {
        write(STDOUT_FILENO, buf, n);
    }
    close(out_pipe[0]);
    int status = 0;
    waitpid(pid, &status, 0);
    return 0;
}
