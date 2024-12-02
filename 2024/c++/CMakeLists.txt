cmake_minimum_required(VERSION 3.10)
project(AdventOfCode)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Include the src folder
include_directories(src)

# Create a static library for PuzzleInput
add_library(PuzzleInputLib STATIC src/PuzzleInput.cpp)

# Collect all solution files (excluding PuzzleInput.cpp)
file(GLOB SOLUTION_FILES "src/[0-9]*.cpp")

# Add an executable for each solution day and link it to the PuzzleInput library
foreach(SOURCE ${SOLUTION_FILES})
    get_filename_component(EXECUTABLE_NAME ${SOURCE} NAME_WE)
    add_executable(${EXECUTABLE_NAME} ${SOURCE})
    target_link_libraries(${EXECUTABLE_NAME} PuzzleInputLib)
endforeach()
