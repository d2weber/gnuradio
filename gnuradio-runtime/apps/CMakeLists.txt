# Copyright 2010-2011,2013 Free Software Foundation, Inc.
#
# This file is part of GNU Radio
#
# SPDX-License-Identifier: GPL-3.0-or-later
#

########################################################################
# Add rust library
########################################################################
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(TARGET_DIR debug)
else ()
    set(TARGET_DIR release)
endif ()

# If the user is cross compiling to a different target, update the location of the target directory
# *Note*: Make sure to install the target with rustup, as well as setting the proper environment variables, such as CC, HOST_CC, CARGO_TARGET_<triple>_LINKER, ...
# See https://doc.rust-lang.org/cargo/reference/environment-variables.html#configuration-environment-variables
if (DEFINED ENV{CARGO_BUILD_TARGET})
    set(TARGET_DIR "$ENV{CARGO_BUILD_TARGET}/${TARGET_DIR}")
endif()

if(ENABLE_LTO)
    set(RUST_FLAGS "-Clinker-plugin-lto" "-Clinker=clang-19" "-Clink-arg=-fuse-ld=lld-19")
endif()

set(lib_file "${CMAKE_SOURCE_DIR}/target/${TARGET_DIR}/${CMAKE_STATIC_LIBRARY_PREFIX}gnuradio_config_info${CMAKE_STATIC_LIBRARY_SUFFIX}")
set(cc_file "${CMAKE_SOURCE_DIR}/target/cxxbridge/gnuradio/gnuradio-runtime/apps/gnuradio-config-info.rs.cc")

add_custom_target(cargo_build-gnuradio-config-info
    COMMAND ${CMAKE_COMMAND} -E env
        RUSTFLAGS="${RUST_FLAGS}"
        cargo build $<$<NOT:$<CONFIG:Debug>>:--release>
    USES_TERMINAL
    BYPRODUCTS ${lib_file}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)

add_library(rs-gnuradio-config-info STATIC ${cc_file})
target_link_libraries(rs-gnuradio-config-info ${lib_file})
target_include_directories(rs-gnuradio-config-info INTERFACE ${CMAKE_SOURCE_DIR}/target/cxxbridge/)
add_dependencies(rs-gnuradio-config-info cargo_build-gnuradio-config-info)

########################################################################
# Setup executables
########################################################################
add_executable(gnuradio-config-info gnuradio-config-info.cc)
target_link_libraries(gnuradio-config-info gnuradio-runtime Boost::program_options rs-gnuradio-config-info)
install(TARGETS gnuradio-config-info DESTINATION ${GR_RUNTIME_DIR})
