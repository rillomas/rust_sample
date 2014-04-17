cmake_minimum_required(VERSION 2.8)
project(opengl_es)
set(cmake_c_flags_release "/W3")
set(es_util_root gles2_book)
set(es_util_src_root ${es_util_root}/src)

include_directories(
	${es_util_root}/include
	${es_util_src_root})

set(es_util_srcs 
	${es_util_src_root}/esShader.c
	${es_util_src_root}/esShapes.c
	${es_util_src_root}/esTransform.c
	${es_util_src_root}/esUtil.c
	${es_util_src_root}/Win32/esUtil_TGA.c
	${es_util_src_root}/Win32/esUtil_win32.c)

add_library(es_util STATIC ${es_util_srcs})
