/* Copyright (C) 2017-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_DISPLAYS_H__
#define __VNC_DISPLAYS_H__

#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Manages the list of displays (that is, monitors) currently attached to the 
 * Server computer.
 *
 * To obtain the current list, call @ref vnc_Server_getDisplayManager to obtain 
 * a @ref vnc_DisplayManager object (a list with a 0-based index). Call 
 * @ref vnc_DisplayManager_selectDisplay with a particular @p index 
 * to remote just that monitor to connected Viewer app users. Specify -1 
 * to remote all monitors (this is the default behavior).
 *
 * To detect monitor changes while the current session is in progress, call 
 * @ref vnc_DisplayManager_setCallback. If a monitor is added or removed, or the 
 * resolution of an existing monitor changes, 
 * vnc_DisplayManager_Callback::displaysChanged returns a new 
 * @ref vnc_DisplayManager object. Call @ref vnc_DisplayManager_selectDisplay 
 * again to remote a monitor from the new list.
 * 
 * Note that functions taking an @p index argument return undefined if the 
 * given value is out of range. In this circumstance, a call to 
 * vnc_getLastError() returns @ref InvalidArgument.
 * 
 * @since 1.5
 */

/**
 * Callback receiving display changes.
 *
 * @see vnc_DisplayManager_setCallback
 */
typedef struct {
  /**
   * Notification that a display has been added or removed, or the resolution 
   * of an existing display has changed.
   *
   * @param displayManager A displayManager object containing a new list 
   * of all the displays.
   *
   * @since 1.5
   */
  void
  (*displaysChanged)(void* userData,
                     vnc_DisplayManager* displayManager);
} vnc_DisplayManager_Callback;

/**
 * Returns the number of displays.
 */
VNC_SDK_API int
vnc_DisplayManager_getDisplayCount(vnc_DisplayManager* displayManager);

/**
 * Registers a callback notifying when displays are added or removed, or the 
 * resolution of an existing display changes.
 *
 * @param displayManager The display manager.
 * @param callback The callback.
 * @param userData Optional custom data to pass to the callback.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_status_t
vnc_DisplayManager_setCallback(vnc_DisplayManager* displayManager,
                               _In_opt_ const vnc_DisplayManager_Callback* callback,
                               void* userData);

/**
 * Chooses a particular display to remote to connected Viewer app users.
 *
 * This replaces any previous selection, so either a single
 * display is remoted or all are.
 *
 * If the selected display is removed, connected Viewers will cease to
 * receive framebuffer updates until the display is replaced or a new
 * display is selected.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the display to remote, 
 * or -1 to remote all displays.
 *
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 *     to get the error code.
 *
 * @retval InvalidArgument
 *     @p index is invalid.
 * @retval NotEnabled
 *     Selecting the display is not permitted at present.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_status_t
vnc_DisplayManager_selectDisplay(vnc_DisplayManager* displayManager,
                                 int index);

/**
 * Gets the index of the currently selected display.
 *
 * @param displayManager The display manager.
 *
 * @return The 0-based index of the display selected to remote,
 *     or -1 for all displays.
 *
 * @since 1.8.0
 */
VNC_SDK_API int
vnc_DisplayManager_getDisplayIndex(vnc_DisplayManager* displayManager);


/**
 * Gets the ID of the display (typically a short string).
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return A unique identifier.
 *
 * @since 1.5
 */
VNC_SDK_API const char*
vnc_DisplayManager_getId(vnc_DisplayManager* displayManager,
                         int index);

/**
 * Gets the name of the display (typically a human-readable string).
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return A friendly name.
 *
 * @since 1.5
 */
VNC_SDK_API const char*
vnc_DisplayManager_getName(vnc_DisplayManager* displayManager,
                           int index);

/**
 * Returns whether this is the primary (or main) display.
 *
 * On Windows, the main display acts as the active desktop, showing 
 * the taskbar and sign in and lock screens. Most applications open 
 * on the main display by default.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return #vnc_true if this is the primary display, otherwise #vnc_false.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_bool_t
vnc_DisplayManager_isPrimary(vnc_DisplayManager* displayManager,
                             int index);

/**
 * Gets the horizontal resolution of the display in pixels.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return A width in pixels.
 *
 * @since 1.5
 */
VNC_SDK_API int
vnc_DisplayManager_getResolutionX(vnc_DisplayManager* displayManager,
                                  int index);

/**
 * Gets the vertical resolution of the display in pixels.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return A height in pixels.
 *
 * @since 1.5
 */
VNC_SDK_API int
vnc_DisplayManager_getResolutionY(vnc_DisplayManager* displayManager,
                                  int index);

/**
 * Gets the horizontal origin of the display in pixels.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return The X coordinate of the top left corner of the display
 * relative to the operating system's virtual map of all the displays.
 *
 * @since 1.5
 */
VNC_SDK_API int
vnc_DisplayManager_getOriginX(vnc_DisplayManager* displayManager,
                              int index);

/**
 * Gets the vertical origin of the display in pixels.
 *
 * @param displayManager The display manager.
 * @param index The 0-based index of the required display.
 *
 * @return The Y coordinate of the top left corner of the display
 * relative to the operating system's virtual map of all the displays.
 *
 * @since 1.5
 */
VNC_SDK_API int
vnc_DisplayManager_getOriginY(vnc_DisplayManager* displayManager,
                              int index);


/**
 *  Returns whether the selectDisplay() call is permitted.
 *
 * @param displayManager The display manager.
 * @return #vnc_true if selectDisplay is permitted, otherwise #vnc_false.
 */
VNC_SDK_API vnc_bool_t
vnc_DisplayManager_isSelectDisplayAllowed(vnc_DisplayManager* displayManager);

/**
 * Specifies whether screen sharing is enabled.
 * If this is the server then this sets whether it shares its screen with
 * viewers.
 * If this is the viewer then this sets whether it requires to see the server's
 * screen.
 *
 * @param displayManager The display manager.
 * @param enable true if screen sharing enabled, false otherwise
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @since 1.8.0
 */
VNC_SDK_API vnc_status_t
vnc_DisplayManager_enable(vnc_DisplayManager* displayManager,
                          vnc_bool_t enable);

/**
 * Returns whether the server is sharing its screens, whether it is being 
 * called from a server or a viewer.
 *
 * @param displayManager The display manager.
 * @return #vnc_true if screens are shared, otherwise #vnc_false.
 *
 * @since 1.8.0
 */
VNC_SDK_API vnc_bool_t
vnc_DisplayManager_isEnabled(vnc_DisplayManager* displayManager);

#ifdef __cplusplus
}
#endif

#endif
