#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

const char* detect_os() {
#ifdef __APPLE__
    return "macos";
#elif __linux__
    return "linux";
#else
    return "unknown";
#endif
}

const char* detect_arch() {
    FILE *fp = popen("uname -m", "r");
    static char arch[64];
    if (!fp) return "unknown";
    fgets(arch, sizeof(arch), fp);
    pclose(fp);
    arch[strcspn(arch, "\n")] = '\0'; // strip newline

    if (strcmp(arch, "x86_64") == 0) return "x86_64";
    if (strcmp(arch, "arm64") == 0 || strcmp(arch, "aarch64") == 0) return "aarch64";
    return "unknown";
}

int main() {
    const char *repo = "myferr/bake";
    const char *install_path = "/usr/local/bin/bake";

    const char *os = detect_os();
    const char *arch = detect_arch();

    if (strcmp(os, "unknown") == 0 || strcmp(arch, "unknown") == 0) {
        fprintf(stderr, "!!! Unsupported platform: %s-%s\n", os, arch);
        return 1;
    }

    char filename[128];
    snprintf(filename, sizeof(filename), "bake-%s-%s", os, arch);

    char url[512];
    snprintf(url, sizeof(url),
        "https://github.com/%s/releases/latest/download/%s",
        repo, filename);

    printf(">>> Detected %s-%s\n", os, arch);
    printf("!!!  Downloading %s...\n", url);

    char cmd[1024];
    snprintf(cmd, sizeof(cmd),
        "curl -L \"%s\" -o /tmp/bake && chmod +x /tmp/bake && sudo mv /tmp/bake %s",
        url, install_path);

    int result = system(cmd);
    if (result != 0) {
        fprintf(stderr, "❌ Installation failed.\n");
        return 1;
    }

    printf("✅ Installed to %s\n", install_path);
    printf("🧪 Run with: bake\n");
    return 0;
}
