/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_ADDON_H__
#define __VNC_ADDON_H__

#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * @brief SDK add-on management. Use the API provided here to manage
 * optional add-ons that will be employed by your application.
 *
 * @since 1.2
 */


/**
 * Enable an SDK add-on by passing in the content of the add-on code,
 * obtained from RealVNC.
 *
 * @param addOnCode The text content of an add-on code.
 *
 * @return #vnc_success if successful. If #vnc_failure is returned then call
 *     vnc_getLastError() to get the error code.
 *
 * @retval InvalidArgument
 *     @p addOnCode is not a valid base64 text string.
 * @retval NotEnabled
 *     The code is not accepted and the add-on is not enabled. Refer to log
 *     output for more information regarding the reason for the code being
 *     rejected.
 * @retval NotSupported
 *     The code is valid but the add-on it enables is not supported on the
 *     platform the application is running on.
 *
 * @since 1.2
 */
VNC_SDK_API vnc_status_t
vnc_enableAddOn(const char* addOnCode);


#ifdef __cplusplus
}
#endif

#endif
