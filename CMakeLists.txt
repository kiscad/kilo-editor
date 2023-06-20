cmake_minimum_required(VERSION 3.18)
project(kilo C)

set(CMAKE_C_STANDARD 11)

add_executable(kilo main.c terminal.c terminal.h error.h error.c input.h input.c output.h output.c append_buffer.h append_buffer.c data.c data.c data.h data.h file.h file.c row.h row.c)
