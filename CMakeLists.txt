cmake_minimum_required(VERSION 3.13.4)
project(FrameLang)

find_package(LLVM REQUIRED CONFIG)

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

# Set your project compile flags.
# E.g. if using the C++ header files
# you will need to enable C++11 support
# for your compiler.
set( CMAKE_CXX_STANDARD 20)

include_directories(${LLVM_INCLUDE_DIRS})
separate_arguments(LLVM_DEFINITIONS_LIST NATIVE_COMMAND ${LLVM_DEFINITIONS})
add_definitions(${LLVM_DEFINITIONS_LIST})

file(GLOB frame_SRC CONFIGURE_DEPENDS "*/*.h" "*/*.hpp" "*/*.cpp")

# Now build our tools
add_executable(frame_compiler ${frame_SRC})

# Link against LLVM libraries
target_link_libraries(frame_compiler -lLLVM)
