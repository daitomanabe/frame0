cmake_minimum_required(VERSION 3.20)
project(frame0_external_cpp_template LANGUAGES CXX)

add_library(frame0_external_cpp_template SHARED
  src/frame0_cpp_external_adapter.cpp
)

target_compile_features(frame0_external_cpp_template PRIVATE cxx_std_17)
target_include_directories(frame0_external_cpp_template PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}/include
  ${CMAKE_CURRENT_SOURCE_DIR}/../../native/frame0_external_c_api
)
