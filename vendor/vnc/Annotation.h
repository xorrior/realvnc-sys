/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_ANNOTATION_H__
#define __VNC_ANNOTATION_H__

#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Enables a Viewer or Server to annotate (that is, draw lines on top of) a
 * Server device screen, without affecting the underlying desktop.
 *
 * To enable a Viewer to annotate, call @ref vnc_Viewer_getAnnotationManager
 * to obtain a @ref vnc_AnnotationManager object. To enable a Server to manage
 * annotations for Viewers, or to annotate itself if desired, call
 * @ref vnc_Server_getAnnotationManager.
 *
 * A Server must grant a connecting Viewer @ref vnc_Server_PermAnnotation
 * in the set of permissions returned by the
 * @ref vnc_Server_SecurityCallback::authenticateUser callback. A Viewer can
 * call @ref vnc_AnnotationManager_setCallback and monitor
 * @ref vnc_AnnotationManager_Callback::availabilityChanged to be notified if
 * annotation availability changes; this might be due to a transient error, but
 * it might be because the Server has revoked annotation permissions mid-session
 * using @ref vnc_Server_setPermissions.
 *
 * A @ref vnc_AnnotationManager controls a pen with a unique color determined by
 * the SDK and a sensible default size, duration and fade time. You can query
 * and change these defaults at any time.
 *
 * Typically, a Viewer or Server annotates in response to an app user moving the
 * mouse cursor or touching the screen. Monitor platform mouse or touch events
 * as appropriate and call @ref vnc_AnnotationManager_movePenTo with @c penDown
 * set to #vnc_true to draw a line from the current position to a new position.
 * To stop drawing, call the same method with @c penDown set to #vnc_false.
 * Note a Viewer might want to suspend calls to @ref vnc_Viewer_sendPointerEvent
 * for the duration in order to stop remote control while drawing operations
 * are in progress.
 *
 * A Viewer can clear its own annotations, or a Server can clear Viewer
 * annotations, or just its own. Note a Viewer's annotations are automatically
 * cleared when it disconnects, and all annotations are cleared if the Server
 * screen resolution changes, for example when an Android device rotates.
 *
 * See the <a href="https://developer.realvnc.com/docs/latest/overview.html">
 * Overview</a> page of the Developer Docs for more information Annotating
 * to a server device screen.
 *
 * @since 1.4
 */

/**
 * Callback receiving annotation-related notifications
 *
 * @see vnc_AnnotationManager_setCallback
 */
typedef struct {
  /**
   * Notification that annotation availability has changed.
   *
   * This optional callback is notified if a call to
   * @ref vnc_AnnotationManager_isAvailable would return a different value.
   *
   * @param annotationManager Belonging to either a Viewer or a Server.
   * @param isAvailable #vnc_true if annotations are currently possible,
   * #vnc_false if not.
   *
   * @since 1.4
   */
  void
  (*availabilityChanged)(void* userData,
                         vnc_AnnotationManager* annotationManager,
                         vnc_bool_t isAvailable);
} vnc_AnnotationManager_Callback;

/**
 * Sets annotation-related callbacks.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param callback The new annotation callback.
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 * to get the error code.
 *
 * @retval InvalidArgument
 *     @p callback is invalid.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_status_t
vnc_AnnotationManager_setCallback(
    vnc_AnnotationManager* annotationManager,
    _In_opt_ const vnc_AnnotationManager_Callback* callback,
    void* userData);

/**
 * Clears particular annotations.
 *
 * For a Viewer, this is all its annotations. For a Server, you can choose to
 * clear its annotations, or those of a particular Viewer.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param fade #vnc_true to fade annotations gradually, or #vnc_false to
 * clear immediately without fading.
 * @param connection For a Server, the Viewer whose annotations to clear,
 * or NULL to clear Server annotations. For a Viewer, this must be NULL.
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 * to get the error code.
 *
 * @retval InvalidArgument
 *     @p annotationManager is invalid or
 *     @p connection is invalid.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_status_t
vnc_AnnotationManager_clear(
    vnc_AnnotationManager* annotationManager,
    vnc_bool_t fade,
    _In_opt_ vnc_Connection* connection);

/**
 * Clears all annotations.
 *
 * For a Server, clears all annotations, including its own.
 * For a Viewer, this is the same as
 * @ref vnc_AnnotationManager_clear.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param fade #vnc_true to fade annotations gradually, or #vnc_false
 * to clear immediately without fading.
 *
 * @since 1.4
 */
VNC_SDK_API void
vnc_AnnotationManager_clearAll(vnc_AnnotationManager* annotationManager, vnc_bool_t fade);

/**
 * Queries whether it is possible to annotate.
 *
 * Note that a Server cannot annotate until app initialization is complete. A
 * Viewer cannot annotate if it is not connected to a Server, if it has not
 * been granted annotation permissions, or if the Server does not support
 * annotations (perhaps because that Server was not created using the
 * VNC SDK).
 *
 * @see vnc_AnnotationManager_Callback::availabilityChanged
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @return #vnc_true if it is possible to annotate, otherwise #vnc_false.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_bool_t
vnc_AnnotationManager_isAvailable(vnc_AnnotationManager* annotationManager);

/**
 * Gets the current pen color.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @return A 32-bit integer in the form AARRGGBB.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_uint32_t
vnc_AnnotationManager_getPenColor(vnc_AnnotationManager* annotationManager);

/**
 * Sets the pen color, determining the color of the annotation line.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param color A 32-bit integer in the form AARRGGBB.
 *
 * @since 1.4
 */
VNC_SDK_API void
vnc_AnnotationManager_setPenColor(vnc_AnnotationManager* annotationManager,
                                  vnc_uint32_t color);

/**
 * Gets the current pen size. Defaults to 10 device independent pixels (DIP).
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @return A number of DIP.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_uint31_t
vnc_AnnotationManager_getPenSize(vnc_AnnotationManager* annotationManager);

/**
 * Sets the pen size, determining the width of the annotation line.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param size A number of DIP.
 *
 * @since 1.4
 */
VNC_SDK_API void
vnc_AnnotationManager_setPenSize(vnc_AnnotationManager* annotationManager, vnc_uint31_t size);

/**
 * Gets how long annotations persist as a solid color for. Defaults to
 * 10000 milliseconds.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @return A number of milliseconds. A value of 0 means annotations start fading
 * immediately. A value of -1 or lower means annotations persist indefinitely
 * unless explicitly or automatically cleared (see the detailed description
 * summary for when this might happen).
 *
 * @since 1.4
 */
VNC_SDK_API int
vnc_AnnotationManager_getPersistDuration(vnc_AnnotationManager* annotationManager);

/**
 * Sets how long annotations persist as a solid color for.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param durationMs A number of milliseconds. Specify 0 to start fading
 * immedaitely. Specify -1 or lower to persist annotations indefinitely unless
 * explicitly or automatically cleared (see the detailed description for when
 * this might happen).
 *
 * @since 1.4
 */
VNC_SDK_API void
vnc_AnnotationManager_setPersistDuration(vnc_AnnotationManager* annotationManager,
                                         int durationMs);

/**
 * Gets how long annotations take to fade. Defaults to
 * 2000 milliseconds.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @return A number of milliseconds. A value of 0 means annotations do not
 * fade but rather disappear immediately after
 * @ref vnc_AnnotationManager_getPersistDuration.
 *
 * @since 1.4
 */
VNC_SDK_API int
vnc_AnnotationManager_getFadeDuration(vnc_AnnotationManager* annotationManager);

/**
 * Sets how long annotations take to fade.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param durationMs A number of milliseconds. Specify 0 to clear annotations
 * immediately after @ref vnc_AnnotationManager_getPersistDuration.
 *
 * @since 1.4
 */
VNC_SDK_API void
vnc_AnnotationManager_setFadeDuration(vnc_AnnotationManager* annotationManager,
                                      int durationMs);

/**
 * Draws a line on the Server screen from the current position to a
 * new position.
 *
 * When you obtain a @ref vnc_AnnotationManager, its
 * co-ordinate system is set to 0,0. This means that before you
 * annotate for the first time, call this method with @p penDown
 * set to #vnc_false to move the pen from 0,0 to the current position
 * without drawing a line. You can then call this method again with
 * @p penDown set to #vnc_true to actually draw from the current position to a
 * new position. Call this method with @p penDown set to
 * #vnc_false to stop drawing.
 *
 * A @ref vnc_AnnotationManager subsequently remembers its current
 * position, so you only need manoevre from 0,0 once. It is only reset to
 * 0,0 if you obtain a new @ref vnc_AnnotationManager,
 * or if the Server screen resolution changes.
 *
 * @param annotationManager Belonging to either a Viewer or a Server.
 * @param x New X co-ordinate.
 * @param y New Y co-ordinate.
 * @param penDown #vnc_true to draw a line from the current position to
 * the new position, or #vnc_false to move the pen without drawing.
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 * to get the error code.
 *
 * @retval InvalidArgument
 *     @p annotationManager is invalid.
 * @retval NotSupported
 *     An attempt was made to annotate with the pen down while annotations
 *     were not available.
 *     @see vnc_AnnotationManager_isAvailable
 *
 * @since 1.4
 */
VNC_SDK_API vnc_status_t
vnc_AnnotationManager_movePenTo(vnc_AnnotationManager* annotationManager,
                                int x,
                                int y,
                                vnc_bool_t penDown);

/** @} */

#ifdef __cplusplus
}
#endif


#endif //__VNC_ANNOTATION_H__
