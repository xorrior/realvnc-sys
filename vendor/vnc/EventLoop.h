/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_EVENTLOOP_H__
#define __VNC_EVENTLOOP_H__

#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * @brief Run the SDK's event loop.
 *
 * The following functions are used for using the VNC SDK's event loop as the
 * the application's event loop.  In this case, the event loop will respond to
 * the events as they occur inside the SDK, invoking callbacks as appropriate,
 * until vnc_EventLoop_stop() is called inside one of the callbacks, at
 * which point vnc_EventLoop_run() exits on the next turn of the event loop.
 *
 * The other way of using the SDK is to use an event loop external to the SDK.
 * On some platforms such as Android, OS X and iOS, the SDK is able to
 * directly register its timers and sockets with the thread's Android or Core
 * Foundation run loop, so simply using the platform's normal event loop will
 * allow the SDK to process I/O automatically.  On other platforms, the SDK's
 * timers and sockets must instead be manually added to the application's
 * event loop; use the functions in vnc/EventLoopFd.h and vnc/EventLoopWin.h to
 * do this.
 *
 * If another thread needs to schedule work for the SDK thread to perform, it
 * should push an object representing the necessary work to a thread-safe queue
 * and then call vnc_EventLoop_stop().  This should cause the SDK thread to
 * complete its call to vnc_EventLoop_run().  The SDK thread should then pop
 * and execute any queued work before looping back to call vnc_EventLoop_run()
 * again, as in the following pseudocode:
 *
 * @code
 * var queue = [];
 * var queueMutex = mutex();
 *
 * // On non-SDK thread:
 * function runOnSdkThread(var runnable) {
 *     try {
 *         lock(queueMutex);
 *         queue.push(runnable);
 *     } finally { unlock(queueMutex); }
 *     vnc_EventLoop_stop();
 * }
 *
 * // On SDK thread:
 * function runSdk() {
 *     var task;
 *     while (true) {
 *         vnc_EventLoop_run();
 *         try {
 *             lock(queueMutex);
 *             while (!queue.empty()) {
 *                 task = queue.pop();
 *                 task();
 *             }
 *         } finally { unlock(queueMutex); }
 *     }
 * }
 * @endcode
 *
 * vnc_EventLoop_run() and vnc_EventLoop_stop() are not available on the HTML5
 * platform.
 */

/**
 * Runs the event loop until vnc_EventLoop_stop() is called.  This function may
 * not be used if the SDK was initialized with @ref vnc_EventLoopType_Android.
 */
VNC_SDK_API void vnc_EventLoop_run(void);

/**
 * Stops the event loop previously started with vnc_EventLoop_run(), causing
 * vnc_EventLoop_run() to return promptly.  This function can be called on the
 * same thread as vnc_EventLoop_run() from inside an SDK callback, or can also
 * be called from another thread.  (This is the only function in the VNC SDK
 * which can safely be called on a thread other than the one which invoked
 * vnc_init()).  After the event loop has been stopped, is it possible to
 * continue it by calling vnc_EventLoop_run() again.  If vnc_EventLoop_stop()
 * is called when vnc_EventLoop_run() is not currently executing, it will exit
 * immediately the next time it is called.  This function may not be used if the
 * SDK was initialized with @ref vnc_EventLoopType_Android.
 */
VNC_SDK_API void vnc_EventLoop_stop(void);

/**
 * Returns a boolean flag indicating whether the event loop should stop,
 * and immediately clears it.
 *
 * The flag is set in response to a call to vnc_EventLoop_stop(),
 * but the change may not take effect until the event loop runs.
 *
 * The flag is cleared automatically when vnc_EventLoop_run() returns.
 *
 * @return #vnc_true if the event loop should stop, #vnc_false otherwise.
 */
VNC_SDK_API vnc_bool_t vnc_EventLoop_shouldStop(void);

#ifdef __cplusplus
}
#endif

#endif
