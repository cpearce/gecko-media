/* List of defines generated by configure. Included with preprocessor flag,
 * -include, to avoid long list of -D defines on the compile command-line.
 * Do not edit.
 */

#ifndef MOZILLA_CONFIG_H
#define MOZILLA_CONFIG_H

#if defined(__clang__)
#pragma clang diagnostic push
#if __has_warning("-Wreserved-id-macro")
#pragma clang diagnostic ignored "-Wreserved-id-macro"
#endif
#endif

/* Expands to all the defines from configure. */
#define A11Y_LOG 1
#define ACCESSIBILITY 1
#define ATK_MAJOR_VERSION 2
#define ATK_MINOR_VERSION 26
#define ATK_REV_VERSION 0
#define BUILD_CTYPES 1
#define CROSS_COMPILE 
#define D_INO d_ino
#define EARLY_BETA_OR_EARLIER 1
#define ENABLE_INTL_API 1
#define ENABLE_SYSTEM_EXTENSION_DIRS 1
#define ENABLE_TESTS 1
#define EXPOSE_INTL_API 1
#define FIREFOX_VERSION 58.0a1
#define FORCE_PR_LOG 1
#define FUNCPROTO 15
#define GDK_VERSION_MAX_ALLOWED GDK_VERSION_3_4
#define GLIB_VERSION_MAX_ALLOWED GLIB_VERSION_2_32
#define GLIB_VERSION_MIN_REQUIRED GLIB_VERSION_2_26
#define GL_PROVIDER_GLX 1
#define GTEST_HAS_RTTI 0
#define HAVE_64BIT_BUILD 1
#define HAVE_ALLOCA_H 1
#define HAVE_BYTESWAP_H 1
#define HAVE_CLOCK_MONOTONIC 1
#define HAVE_CPUID_H 1
#define HAVE_DIRENT_H 1
#define HAVE_DLADDR 1
#define HAVE_DLOPEN 1
#define HAVE_FONTCONFIG_FCFREETYPE_H 1
#define HAVE_FT_BITMAP_SIZE_Y_PPEM 1
#define HAVE_FT_GLYPHSLOT_EMBOLDEN 1
#define HAVE_FT_LOAD_SFNT_TABLE 1
#define HAVE_GETOPT_H 1
#define HAVE_GMTIME_R 1
#define HAVE_I18N_LC_MESSAGES 1
#define HAVE_INTTYPES_H 1
#define HAVE_LANGINFO_CODESET 1
#define HAVE_LCHOWN 1
#define HAVE_LIBXSS 1
#define HAVE_LINUX_IF_ADDR_H 1
#define HAVE_LINUX_PERF_EVENT_H 1
#define HAVE_LINUX_QUOTA_H 1
#define HAVE_LINUX_RTNETLINK_H 1
#define HAVE_LOCALECONV 1
#define HAVE_LOCALTIME_R 1
#define HAVE_LSTAT64 1
#define HAVE_MALLINFO 1
#define HAVE_MALLOC_H 1
#define HAVE_MALLOC_USABLE_SIZE 1
#define HAVE_MEMALIGN 1
#define HAVE_MEMMEM 1
#define HAVE_NETINET_IN_H 1
#define HAVE_NL_TYPES_H 1
#define HAVE_POSIX_FADVISE 1
#define HAVE_POSIX_FALLOCATE 1
#define HAVE_POSIX_MEMALIGN 1
#define HAVE_PTHREAD_H 1
#define HAVE_RES_NINIT 1
#define HAVE_SETPRIORITY 1
#define HAVE_STAT64 1
#define HAVE_STDINT_H 1
#define HAVE_STRERROR 1
#define HAVE_STRNDUP 1
#define HAVE_SYSCALL 1
#define HAVE_SYS_QUEUE_H 1
#define HAVE_SYS_QUOTA_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_THREAD_TLS_KEYWORD 1
#define HAVE_TRUNCATE64 1
#define HAVE_UNISTD_H 1
#define HAVE_VALLOC 1
#define HAVE_VA_COPY 1
#define HAVE_VA_LIST_AS_ARRAY 1
#define HAVE_VISIBILITY_ATTRIBUTE 1
#define HAVE_VISIBILITY_HIDDEN_ATTRIBUTE 1
#define HAVE__UNWIND_BACKTRACE 1
#define HAVE___CXA_DEMANGLE 1
#define JS_DEFAULT_JITREPORT_GRANULARITY 3
#define MALLOC_H <malloc.h>
#define MALLOC_USABLE_SIZE_CONST_PTR 
#define MOZILLA_UAVERSION "58.0"
#define MOZILLA_VERSION "58.0a1"
#define MOZILLA_VERSION_U 58.0a1
#define MOZ_ACCESSIBILITY_ATK 1
#define MOZ_ADDON_SIGNING 1
#define MOZ_ALLOW_LEGACY_EXTENSIONS 1
#define MOZ_APP_UA_NAME ""
#define MOZ_APP_UA_VERSION "58.0a1"
#define MOZ_AV1 1
#define MOZ_BUILD_APP browser
#define MOZ_BUILD_WEBRENDER 1
#define MOZ_BUNDLED_FONTS 1
#define MOZ_CONTENT_SANDBOX 1
#define MOZ_CRASHREPORTER 1
#define MOZ_DATA_REPORTING 1
#define MOZ_DEBUG_RUST 1
#define MOZ_DEMANGLE_SYMBOLS 1
#define MOZ_DISTRIBUTION_ID "org.mozilla"
#define MOZ_DLL_SUFFIX ".so"
#define MOZ_DUMP_PAINTING 1
#define MOZ_ENABLE_DBUS 1
#define MOZ_ENABLE_GCONF 1
#define MOZ_ENABLE_SIGNMAR 1
#define MOZ_ENABLE_SKIA 1
#define MOZ_ENABLE_SKIA_PDF 1
#define MOZ_ENABLE_SKIA_PDF_SFNTLY 1
#define MOZ_ENABLE_XREMOTE 1
#define MOZ_FEEDS 1
#define MOZ_FFMPEG 1
#define MOZ_FFVPX 1
#define MOZ_FMP4 1
#define MOZ_GECKO_PROFILER 1
#define MOZ_GLUE_IN_PROGRAM 1
#define MOZ_GMP_SANDBOX 1
#define MOZ_INSTRUMENT_EVENT_LOOP 1
#define MOZ_LIBAV_FFT 1
#define MOZ_LOGGING 1
#define MOZ_MACBUNDLE_ID org.mozilla.nightlydebug
#define MOZ_MEMORY 1
#define MOZ_PEERCONNECTION 1
#define MOZ_PERMISSIONS 1
#define MOZ_PHOENIX 1
#define MOZ_PLACES 1
#define MOZ_PROFILING 1
#define MOZ_PULSEAUDIO 1
#define MOZ_RAW 1
#define MOZ_REFLOW_PERF 1
#define MOZ_REFLOW_PERF_DSP 1
#define MOZ_REPLACE_MALLOC 1
#define MOZ_RUST_URLPARSE 1
#define MOZ_SAMPLE_TYPE_FLOAT32 1
#define MOZ_SANDBOX 1
#define MOZ_SCTP 1
#define MOZ_SECUREELEMENT 1
#define MOZ_SERVICES_HEALTHREPORT 1
#define MOZ_SRTP 1
#define MOZ_STATIC_JS 1
#define MOZ_STYLO 1
#define MOZ_STYLO_ENABLE 1
#define MOZ_TREE_CAIRO 1
#define MOZ_TREE_PIXMAN 1
#define MOZ_UPDATER 1
#define MOZ_UPDATE_CHANNEL default
#define MOZ_USER_DIR ".mozilla"
#define MOZ_VORBIS 1
#define MOZ_VPX_NO_MEM_REPORTING 1
#define MOZ_VTUNE 1
#define MOZ_WEBM_ENCODER 1
#define MOZ_WEBRTC 1
#define MOZ_WEBRTC_ASSERT_ALWAYS 1
#define MOZ_WEBRTC_SIGNALING 1
#define MOZ_WEBSPEECH 1
#define MOZ_WEBSPEECH_TEST_BACKEND 1
#define MOZ_WIDGET_GTK 3
#define MOZ_X11 1
#define MOZ_XUL 1
#define NIGHTLY_BUILD 1
#define NO_NSPR_10_SUPPORT 1
#define NS_PRINTING 1
#define NS_PRINT_PREVIEW 1
#define STATIC_JS_API 1
#define STDC_HEADERS 1
#define TARGET_XPCOM_ABI "x86_64-gcc3"
#define USE_SKIA 1
#define USE_SKIA_GPU 1
#define U_STATIC_IMPLEMENTATION 1
#define U_USING_ICU_NAMESPACE 0
#define VA_COPY va_copy
#define VPX_X86_ASM 1
#define XP_LINUX 1
#define XP_UNIX 1
#define _REENTRANT 1

/*
 * The c99 defining the limit macros (UINT32_MAX for example), says:
 *
 *   C++ implementations should define these macros only when
 *   __STDC_LIMIT_MACROS is defined before <stdint.h> is included.
 *
 * The same also occurs with __STDC_CONSTANT_MACROS for the constant macros
 * (INT8_C for example) used to specify a literal constant of the proper type,
 * and with __STDC_FORMAT_MACROS for the format macros (PRId32 for example) used
 * with the fprintf function family.
 */
#define __STDC_LIMIT_MACROS
#define __STDC_CONSTANT_MACROS
#if !defined(__STDC_FORMAT_MACROS)
#define __STDC_FORMAT_MACROS
#endif

#if defined(__clang__)
#pragma clang diagnostic pop
#endif

/*
 * Force-include Char16.h in order to define PRUnichar as char16_t everywhere.
 * Note that this should be the first #include to make sure that prtypes.h does
 * not attempt to define PRUnichar.  This includes the following hunspell-specific
 * includes.
 */
#if !defined(__ASSEMBLER__)
#include "mozilla/Char16.h"
#endif

/*
 * Force-include hunspell_alloc_hooks.h and hunspell_fopen_hooks.h for hunspell,
 * so that we don't need to modify them directly.
 *
 * HUNSPELL_STATIC is defined in extensions/spellcheck/hunspell/src/Makefile.in,
 * unless --enable-system-hunspell is defined.
 */
#if defined(HUNSPELL_STATIC)
#include "hunspell_alloc_hooks.h"
#include "hunspell_fopen_hooks.h"
#endif

/*
 * Force-include sdkdecls.h for building the chromium sandbox code.
 *
 * CHROMIUM_SANDBOX_BUILD is defined in security/sandbox/moz.build.
 * Note that this include path relies on the LOCAL_INCLUDES in that file.
 */
#if defined(CHROMIUM_SANDBOX_BUILD) && defined(XP_WIN)
#include "base/win/sdkdecls.h"
#endif

/*
 * Enable audioipc support for CubebUtils on Linux.
 */
#if defined(XP_LINUX) && !defined(MOZ_WIDGET_ANDROID)
#define MOZ_CUBEB_REMOTING
#endif


#endif /* MOZILLA_CONFIG_H */
