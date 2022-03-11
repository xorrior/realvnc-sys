/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_VERSION_H__
#define __VNC_VERSION_H__

#include "Common.h"

/**
 * @file
 * Obtain SDK version information.
 *
 * SDK versioning follows the <a href="http://semver.org/">Semantic
 * Versioning</a> conventions:
 *
 *  - A change in major number indicates an incompatible change.
 *  - A change in minor number only indicates extra functionality added in a
 *    backwards-compatible manner, preserving source compatibility but requiring
 *    recompilation against the new headers.
 *  - A change in patch number only indicates backwards-compatible bug fixes,
 *    preserving binary compatibility.
 */

/** The compile-time major version number of the SDK. */
#define VNC_SDK_MAJOR_VERSION 1
/** The compile-time minor version number of the SDK. */
#define VNC_SDK_MINOR_VERSION 9
/** The compile-time patch version number of the SDK. */
#define VNC_SDK_PATCH_VERSION 1
/** The compile-time build number of the SDK. */
#define VNC_SDK_BUILD_NUMBER 46075

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Returns the runtime major version number of the SDK.
 */
VNC_SDK_API int vnc_getMajorVersion(void);

/**
 * Returns the runtime minor version number of the SDK.
 */
VNC_SDK_API int vnc_getMinorVersion(void);

/**
 * Returns the runtime patch version number of the SDK.
 */
VNC_SDK_API int vnc_getPatchVersion(void);

/**
 * Returns the runtime build number of the SDK.
 */
VNC_SDK_API int vnc_getBuildNumber(void);

#ifdef __cplusplus
}
#endif

#endif
