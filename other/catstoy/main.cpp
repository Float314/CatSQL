/* 
    SPDX-License-Identifier: FSFAP
    CatsToy : CatSQL Helper program 
    Copyright (c) 2026, Float314

    Copying and distribution of this file, with or without modification, 
    are permitted in any medium without royalty, provided the copyright
    notice and this notice are preserved. This file is offered as-is,
    without any warranty.

*/
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <exception>
#include <fstream>
#include <iostream>
#include <stdexcept>
#include <unistd.h>
#include <sys/wait.h>
#include <string>
#include <format>
#include <print>

const char* executable = "./target/debug/CatSQL";

int main(int argc, char* argv[]) {
    try {
    if (argc < 1) {
        std::cout << "Too Less Arguments to begin with!"; // holy rust effect... 
        std::cout << "\nUsage: catstoy [argument]"
        ;
        std::exit(-1);
    }

    if (std::string(argv[1]) == "build") {
        pid_t pid = fork();

        if (pid < 0) {
            printf("Fork failed!");
            std::exit(-2);
        }
        else if (pid == 0) {
            char* args[] = {"cargo", "build", nullptr};
            execvp("cargo", args);

            perror("something bad happened!");
            std::exit(-3);
        }
        else {
            int status;
            waitpid(pid, &status, 0);
            printf("success! build completed!");
        }
    }
    else if (std::string(argv[1]) == "run") {
        std::system(executable);
    } 
    else if (std::string(argv[1]) == "push") {
        if (std::system("git push origin master") != 0) {
            throw std::runtime_error("git push failed!");
        } else {
            printf("Success!");
        };
    }
    else if (std::string(argv[1]) == "tag") {
        if (argc < 2) {
            printf("Too low argument count!");
            std::exit(1);
        }
        else {
            std::string command = "git tag -a " + std::string(argv[2]);
            system(command.c_str()); // see i still have all that info

            auto file = std::ofstream("VERSION.txt");
            file << std::string(argv[2]);
        }
    }
    else {
        printf("invalid code");
        std::exit(9);
    }
} catch (const std::runtime_error& e) {
    std::cout << e.what() << std::endl;
}
}