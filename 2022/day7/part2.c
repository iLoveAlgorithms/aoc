#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <dirent.h>

#define MAX_LINE_LENGTH 100
#define SPACE " "

static int findSmallestDelete(int currSmallest);
static int dirSize(void);
static void removeNewLine(char *line);

int main(void) {
    FILE *input = fopen("input.txt", "r");
    mkdir("runCmds", 0777);
    chdir("runCmds");
    char command[MAX_LINE_LENGTH + 1];
    while (fgets(command, MAX_LINE_LENGTH + 1, input) != NULL) {
        removeNewLine(command);
        char *token = strtok(command, SPACE);

        if (strcmp(token, "$") == 0) {
            token = strtok(NULL, SPACE);
            if (strcmp(token, "cd") == 0) {
                token = strtok(NULL, SPACE);
                if (strcmp(token, "/") != 0) {
                    chdir(token);
                }
            }
        } else if (strcmp(token, "dir") == 0) {
            token = strtok(NULL, SPACE);
            mkdir(token, 0777);
        
        } else {
            char *value = token;
            token = strtok(NULL, SPACE);
            FILE *writeFile = fopen(token, "w");
            fprintf(writeFile, "%s", value);
            fclose(writeFile);
        }
    }
    fclose(input);

    char *home = getenv("HOME");
    chdir(home);
    chdir("aoc2022/day7/runCmds");
    int smallest = findSmallestDelete(70000000);

    printf("%d\n", smallest);
    return 0;
}

static int findSmallestDelete(int currSmallest) {
    int newSmallest = currSmallest;
    struct dirent *dir_entry;
    DIR *currdir = opendir(".");


    while ((dir_entry = readdir(currdir)) != NULL) {

        // Use recurstion to find subdirectories
        if (
            (dir_entry->d_type == DT_DIR) && 
            (strcmp(dir_entry->d_name, ".") != 0) && 
            (strcmp(dir_entry->d_name, "..") != 0) 
        ) {
            chdir(dir_entry->d_name);
            newSmallest = findSmallestDelete(newSmallest);
            chdir("..");
        }
        
    }
    if (dirSize() >= 1412830) {
        newSmallest = dirSize() < newSmallest ? dirSize() : newSmallest;
    }

    closedir(currdir);
    return newSmallest;
}

static int dirSize(void) {
    int totalSize = 0;
    struct dirent *dir_entry;
    DIR *currdir = opendir(".");

    while ((dir_entry = readdir(currdir)) != NULL) {

        if (
            (dir_entry->d_type == DT_DIR) && 
            (strcmp(dir_entry->d_name, ".") != 0) && 
            (strcmp(dir_entry->d_name, "..") != 0) 
        ) {
            chdir(dir_entry->d_name);
            totalSize += dirSize();
            chdir("..");
        }

        if (dir_entry->d_type == DT_REG) {
            FILE *fp = fopen(dir_entry->d_name, "r");
            char sizeStr[MAX_LINE_LENGTH + 1];
            fgets(sizeStr, MAX_LINE_LENGTH + 1, fp);
            totalSize += strtol(sizeStr, NULL, 10);
            fclose(fp);
        }
    }
    closedir(currdir);

    return totalSize;
}

static void removeNewLine(char *line) {
    for (int i = 0; line[i] != '\0'; i++) {
        if (line[i] == '\n') {
            line[i] = '\0';
        }
    }
}