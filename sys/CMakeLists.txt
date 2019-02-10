cmake_minimum_required (VERSION 3.0)
find_package(PkgConfig REQUIRED)

project(hodges)

# Search for ffmpeg libraries
pkg_check_modules(LIBAVDEVICE REQUIRED libavdevice)
pkg_check_modules(LIBAVFORMAT REQUIRED libavformat)
pkg_check_modules(LIBAVFILTER REQUIRED libavfilter)
pkg_check_modules(LIBAVCODEC REQUIRED libavcodec)
pkg_check_modules(LIBSWRESAMPLE REQUIRED libswresample)
pkg_check_modules(LIBSWSCALE REQUIRED libswscale)
pkg_check_modules(LIBAVUTIL REQUIRED libavutil)

set(CMAKE_CXX_COMPILER clang)
# Decide on C/C++ standards
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -v -O3")



# Add the hodges library
add_library(hodges STATIC
    ${CMAKE_SOURCE_DIR}/src/libhodges.c
)
target_include_directories(hodges
    PUBLIC
    ${CMAKE_SOURCE_DIR}/include
)

MESSAGE(STATUS "Libavfilter: " ${LIBAVFILTER_INCLUDE_DIRS})

TARGET_LINK_LIBRARIES(hodges PUBLIC 
    ${LIBAVDEVICE_LDFLAGS} 
    ${LIBAVFORMAT_LDFLAGS}
    ${LIBAVFILTER_LDFLAGS}
    ${LIBAVCODEC_LDFLAGS}
    ${LIBSWRESAMPLE_LDFLAGS}
    ${LIBSWSCALE_LDFLAGS}
    ${LIBAVUTIL_LDFLAGS}
)
TARGET_INCLUDE_DIRECTORIES(hodges PUBLIC
    ${LIBAVDEVICE_INCLUDE_DIRS}
    ${LIBAVFORMAT_INCLUDE_DIRS}
    ${LIBAVFILTER_INCLUDE_DIRS}
    ${LIBAVCODEC_INCLUDE_DIRS}
    ${LIBSWRESAMPLE_INCLUDE_DIRS}
    ${LIBSWSCALE_INCLUDE_DIRS}
    ${LIBAVUTIL_INCLUDE_DIRS}
)
TARGET_COMPILE_OPTIONS(hodges PUBLIC
    ${LIBAVDEVICE_CFLAGS_OTHER}
    ${LIBAVFORMAT_CFLAGS_OTHER}
    ${LIBAVFILTER_CFLAGS_OTHER}
    ${LIBAVCODEC_CFLAGS_OTHER}
    ${LIBSWRESAMPLE_CFLAGS_OTHER}
    ${LIBSWSCALE_CFLAGS_OTHER}
    ${LIBAVUTIL_CFLAGS_OTHER}
)
set_target_properties(hodges PROPERTIES PUBLIC_HEADER "${hodges_headers}")
set_target_properties(hodges PROPERTIES LINKER_LANGUAGE C)

install(TARGETS hodges
    ARCHIVE  DESTINATION ${CMAKE_INSTALL_PREFIX}
    LIBRARY  DESTINATION ${CMAKE_INSTALL_PREFIX}
    RUNTIME  DESTINATION ${CMAKE_INSTALL_PREFIX})  # This is for Windows
install(DIRECTORY ${CMAKE_SOURCE_DIR}/include/ DESTINATION ${CMAKE_INSTALL_PREFIX})

# Add an example app depending on talamel
add_executable(pcmdumpc apps/pcmdumpc.c)
add_dependencies(pcmdumpc hodges)
set_source_files_properties(pcmdumpc LANGUAGE C)
set_target_properties(pcmdumpc PROPERTIES LINKER_LANGUAGE C)
target_link_libraries(pcmdumpc hodges)

add_executable(pcmdumpf apps/pcmdumpf.c)
add_dependencies(pcmdumpf hodges)
set_source_files_properties(pcmdumpf LANGUAGE C)
set_target_properties(pcmdumpf PROPERTIES LINKER_LANGUAGE C)
target_link_libraries(pcmdumpf hodges)

add_executable(pcmdumpbf apps/pcmdumpbf.c)
add_dependencies(pcmdumpbf hodges)
set_source_files_properties(pcmdumpbf LANGUAGE C)
set_target_properties(pcmdumpbf PROPERTIES LINKER_LANGUAGE C)
target_link_libraries(pcmdumpbf hodges)