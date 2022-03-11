/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_SERVER_H__
#define __VNC_SERVER_H__

#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Creates a VNC-compatible Server app for a computer you want to control.
 *
 * Capturing the device screen and injecting input is handled by @c vncagent,
 * an executable provided with the SDK.  You must bundle this executable with
 * your Server; you can find it in the
 * <tt>\<vnc-sdk\>/libs/\<architecture\></tt> directory.
 *
 * There are two ways in which a Server can operate: as a regular application,
 * following a normal application lifecycle (i.e. exiting when the user logs out
 * of the desktop session); or as a service, which can continue running across
 * sessions and at the login screen. In either case, the SDK must first be
 * initialized with a call to vnc_init().
 *
 * To create a Server as a regular application, use vnc_Server_create(), and for
 * a service, use vnc_Server_createService(). In either case, you can optionally
 * specify a path to (or a new file name for) @c vncagent.  Note you must also
 * call one of the methods in @ref DataStore.h to enable the Server to persist
 * the automatically-generated RSA public/private key pair and other important
 * information, and you can optionally call a method in Logger.h to set up
 * logging. Note that methods in DataStore.h and Logger.h may be called before
 * calling vnc_init().
 *
 * Next, populate a @ref vnc_Server_SecurityCallback structure with functions
 * used to verify credentials presented by a connecting Viewer, and pass it
 * into a call to vnc_Server_setSecurityCallback().
 *
 * Typically, a Server joins VNC Cloud and listens for incoming connections
 * from one or more Viewers.  To start your Server listening, call
 * vnc_Server_getConnectionHandler() to obtain a @ref vnc_ConnectionHandler,
 * and pass that into a call to vnc_CloudListener_create() along with a
 * @c listen Cloud address and password.
 *
 * (Note that you can reverse this methodology and have your Server connect to
 * listening Viewers if you wish.  To see how to do this, see Viewer.h.)
 *
 * When a new viewer connection arrives at the server, any defined
 * vnc_Server_SecurityCallback::verifyPeer callback will be called to allow the
 * peer's identity to be verified (in addition to the verification already
 * carried out by VNC Cloud).  Next,
 * vnc_Server_SecurityCallback::isUserNameRequired and
 * vnc_Server_SecurityCallback::isPasswordRequired will be called to determine
 * what type of authentication credentials the viewer should be prompted for.
 * Once the viewer provides credentials,
 * vnc_Server_SecurityCallback::authenticateUser is called to allow the
 * credentials to be verified, and appropriate @ref vnc_Server_Permissions
 * granted for the connection.
 *
 * Note that by default a Viewer consistently failing to authenticate
 * is blacklisted to guard against brute-force and dictionary attacks; call
 * vnc_Server_setBlacklist() to configure frequency and timeouts.
 *
 * Once authenticated, any defined
 * vnc_Server_ConnectionCallback::connectionStarted callback is called, and the
 * server starts remoting the display, and injecting input events received from
 * the client.  Text copied to the clipboard on the device is sent to the
 * viewer, and any clipboard text sent by the viewer is made available to
 * applications on the device's clipboard (assuming the
 * @ref vnc_Server_PermClipboard permission has been granted).
 *
 * The connection will end if vnc_Server_disconnect() or
 * vnc_Server_disconnectAll() is called, or may end due to an action
 * on the Viewer, or due to some other event such as a network failure.  When
 * the connection ends, the
 * vnc_Server_ConnectionCallback::connectionEnded callback will be called.
 *
 * To stop your Server listening and disconnect from VNC Cloud, call
 * vnc_CloudListener_destroy().  To destroy your Server, call
 * vnc_Server_destroy().
 *
 * To make sure the SDK is shutdown and any resources are cleaned up, call
 * vnc_shutdown().
 */

/**
 * Enumeration of screen capture methods.
 *
 * @see vnc_Server_setCaptureMethod()
 */
typedef enum {
  /**
   * Use the optimal capture method for the platform.  This is the default.
   */
  vnc_Server_CaptureOptimal = 0,
  /**
   * Use a fallback capture method under Windows and Linux.  This is likely to
   * be slower, but may solve rendering glitches with certain third-party
   * applications.  Not supported under Mac OS X.
   */
  vnc_Server_CaptureFallback,
} vnc_Server_CaptureMethod;

/**
 * Enumeration of session permissions that can be granted to a connecting
 * Viewer.  (Flags can be combined into a mask.)
 *
 * @see vnc_Server_SecurityCallback::authenticateUser
 */
typedef enum {
  /** Grants a Viewer permission to see the screen.  Rarely omitted. */
  vnc_Server_PermView = 0x01,
  /** Grants a Viewer permission to inject keyboard events. */
  vnc_Server_PermKeyboard = 0x02,
  /** Grants a Viewer permission to inject mouse and touch events. */
  vnc_Server_PermPointer = 0x04,
  /** Grants a Viewer permission to copy and paste text to and from the Server. */
  vnc_Server_PermClipboard = 0x08,
  /** Grants a Viewer permission to draw on the Server screen. */
  vnc_Server_PermAnnotation = 0x10,
  /** Grants a Viewer pemission to select the Server's display. */
  vnc_Server_PermSelectDisplay = 0x20,
  /** Grants a Viewer pemission to listen to the Server. */
  vnc_Server_PermListen = 0x40,
  /** Grants a Viewer a full set of permissions (that is, all of the above). */
  vnc_Server_PermAll = 0x7fffffff
} vnc_Server_Permissions;

/**
 * Callback receiving connection-related notifications for a Server.
 *
 * @see vnc_Server_setConnectionCallback().
 */
typedef struct {
  /**
   * Notification that a connection has successfully started.
   *
   * This callback is notified when a Viewer has successfully authenticated and
   * been granted a set of session permissions.
   *
   * It is optional to define a callback function of this type.  Note that this
   * callback is the only way to obtain a pointer to the @ref vnc_Connection
   * object identifying this Viewer, required by some method calls.
   *
   * This @ref vnc_Connection object is guaranteed to be valid until
   * vnc_Server_ConnectionCallback::connectionEnded notifies you that it has
   * been destroyed.
   *
   * The connection count, as returned by vnc_Server_getConnectionCount(), is
   * incremented @em before this callback is called.
   *
   * @param connection The new connection.
   */
  void
  (*connectionStarted)(void* userData,
                       vnc_Server* server,
                       vnc_Connection* connection);

  /**
   * Notification that a connection has ended.
   *
   * A callback of this type is made when a Viewer explicitly disconnects from
   * the Server, or a VNC Cloud or other network error has occurred.
   *
   * Note this callback function is called even if you call
   * vnc_Server_disconnect() or vnc_Server_disconnectAll() to terminate
   * connections yourself.
   *
   * Only connections which have successfully authenticated and received a
   * vnc_Server_ConnectionCallback::connectionStarted notification are notified
   * throw this ended notification.
   *
   * It is optional to define a callback function of this type, but note that
   * memory corruption will occur if you call vnc_Server_disconnect() on a
   * freed @ref vnc_Connection, so you must use this callback if you maintain a
   * list of connected viewers.
   *
   * The connection count, as returned by vnc_Server_getConnectionCount(), is
   * decremented @em after this callback is called.
   *
   * @param connection The viewer connection which has ended.
   */
  void
  (*connectionEnded)(void* userData,
                     vnc_Server* server,
                     vnc_Connection* connection);
} vnc_Server_ConnectionCallback;

/**
 * Callback receiving security-related notifications for a Server.
 *
 * @see vnc_Server_setSecurityCallback()
 */
typedef struct {
  /**
   * Verify a Viewer's cryptographic identity.
   *
   * This callback is made when a Viewer makes a connection, but before
   * authentication.
   *
   * It is optional to define a callback function of this type.  Note that
   * VNC Cloud mutually verifies peer identities for you, so this check
   * is for peace-of-mind only (and only likely to be useful if you have
   * out-of-band access to a list of all potential Viewer identities).
   *
   * @param connection The new connection.  Note that if the connection does not
   *     subsequently authenticate itself, then the
   *     vnc_Server_ConnectionCallback::connectionEnded notification will not be
   *     called, therefore you should not cache or store the @ref vnc_Connection
   *     pointer at this stage.
   * @param viewerHexFingerprint A human-readable hexadecimal representation
   *     of a 2048-bit RSA public key hash.
   * @param viewerRsaPublic The Viewer's public key.  See
   *     vnc_RsaKey_Callback::detailsReady for the format.
   *
   * @return #vnc_true to continue the connection or #vnc_false to terminate it.
   */
  vnc_bool_t
  (*verifyPeer)(void* userData,
                vnc_Server* server,
                vnc_Connection* connection,
                const char* viewerHexFingerprint,
                const vnc_DataBuffer* viewerRsaPublic);

  /**
   * Decide if a connecting Viewer is required to provide a user name.
   *
   * This callback is made when a Viewer makes a connection, after peer identity
   * verification has succeeded.
   *
   * It is optional to define a callback function of this type, but note that
   * if you do not, a connecting Viewer is required to provide a user name.
   *
   * In your implementation, you should return #vnc_false if there is no need
   * for a Viewer to provide a user name, and #vnc_true if there is.
   *
   * @param connection The incoming connection.  Note that if the connection
   *     does not subsequently authenticate itself, then the
   *     vnc_Server_ConnectionCallback::connectionEnded notification will not be
   *     called, therefore you should not cache or store the @ref vnc_Connection
   *     pointer at this stage.
   *
   * @return #vnc_false if a user name is not required, else #vnc_true.
   */
  vnc_bool_t
  (*isUserNameRequired)(void* userData,
                        vnc_Server* server,
                        vnc_Connection* connection);

  /**
   * Decide if a connecting Viewer is required to provide a password.
   *
   * This callback is made when a Viewer makes a connection, after peer identity
   * verification has succeeded.
   *
   * It is optional to define a callback function of this type, but note that
   * if you do not, a connecting Viewer is required to provide a password.
   *
   * In your implementation, you should return #vnc_false if there is no need
   * for a Viewer to provide a password, and #vnc_true if there is.
   *
   * @param connection The incoming connection.  Note that if the connection
   *     does not subsequently authenticate itself, then the
   *     vnc_Server_ConnectionCallback::connectionEnded notification will not be
   *     called, therefore you should not cache or store the @ref vnc_Connection
   *     pointer at this stage.
   *
   * @return #vnc_false if a password is not required, else #vnc_true.
   */
  vnc_bool_t
  (*isPasswordRequired)(void* userData,
                        vnc_Server* server,
                        vnc_Connection* connection);

  /**
   * Authenticate credentials presented by a Viewer.
   *
   * This callback is made when a Viewer has sent credentials to be
   * authenticated, after the Server has requested the details returned by the
   * vnc_Server_SecurityCallback::isUserNameRequired and
   * vnc_Server_SecurityCallback::isPasswordRequired callbacks.
   *
   * It is mandatory to define this callback function.
   *
   * In your implementation, you should authenticate @p username and
   * @p password, and either return 0 to terminate the
   * connection, or any combination of the @ref vnc_Server_Permissions flags to
   * simultaneously approve the connection and grant a set of session
   * permissions.
   *
   * Please see the <a href="http://www.securecoding.cert.org">CERT Coding
   * Standards</a> for guidelines on handling passwords in reusable memory, in
   * particular sections MSC18-C and MEM03-C.
   *
   * @param connection The incoming connection.
   * @param username The user name provided by the Viewer.  This is provided
   *     according to whether vnc_Server_SecurityCallback::isUserNameRequired
   *     requested it (provided by default).
   * @param password The password provided by the Viewer.  This is provided
   *     according to whether vnc_Server_SecurityCallback::isPasswordRequired
   *     requested it (provided by default).
   *
   * @return Any combination of the @ref vnc_Server_Permissions flags, or 0
   *     (no permissions) to reject the connection.
   */
  int
  (*authenticateUser)(void* userData,
                      vnc_Server* server,
                      vnc_Connection* connection,
                      const char* username,
                      const char* password);

} vnc_Server_SecurityCallback;


/**
 * Callback for handling input events.
 *
 * @see vnc_Server_setInputEventsCallback()
 */
 typedef struct {
  /**
   * Handle pointer events manually.
   *
   * This callback is made when a Viewer has sent pointer events
   * to the server. Return false to have the Server handle the pointer
   * event as normal, or true to indicate it has been handled and
   * should therefore cancel the default handling.
   *
   * @param x The X position of the pointer event.
   * @param y The Y position of the pointer event.
   * @param buttonMask Bitmask of which buttons are down on the pointing device. 
   *                   The first bit is the main button, the second bit is the secondary button, etc ...
   * @param isRelative #vnc_true if the (x,y) position is relative to the last pointer position
   *                   #vnc_false if the position is absolute.
   *
   * @return #vnc_true if the pointer-event is to be handled by the SDK caller (i.e. override the behaviour),
   *         #vnc_false if the pointer-event is to be handled by the Server as normal (default behaviour).
   */
   vnc_bool_t
   (*pointerEventCallback)(void* userData,
                           vnc_Server* server,
                           int x,
                           int y, 
                           int buttonMask,
                           vnc_bool_t isRelative);

  /**
   * Handle keypress events manually.
   *
   * This callback is made when a Viewer has sent keypress events
   * to the server. Return false to have the Server handle the keypress
   * event as normal, or true to indicate it has been handled and
   * should cancel the default handling.
   *
   * @param key Key pressed from the Viewer. Expressed as an X11 keysym code. See keyboard.h.
   * @param down True if this is the "down" event, false if this is the "up" event.
   *
   * @return #vnc_true if the keypress event is to be handled by the SDK caller (i.e. override the behaviour),
   *         #vnc_false if the keypress event is to be handled by the Server as normal (default behaviour).
   */
   vnc_bool_t
   (*keyEventCallback)(void* userData,
                       vnc_Server* server,
                       int key,
                       vnc_bool_t down);

} vnc_Server_InputEventsCallback;

/**
 * Callback receiving agent-related notifications for a Server.
 *
 * @see vnc_Server_setAgentCallback().
 *
 * @since 1.1
 */
typedef struct {
  /**
   * Notification that the agent has started.
   *
   * Note that the agent is normally started automatically as soon as the server
   * is created via vnc_Server_create() or vnc_Server_createService().
   *
   * This callback is optional.
   */
  void
  (*agentStarted)(void* userData,
                  vnc_Server* server);

  /**
   * Notification that the agent has stopped.
   *
   * This may indicate that the user has logged out or (on Linux) the X server
   * has stopped.
   *
   * This callback is optional.
   */
  void
  (*agentStopped)(void* userData,
                  vnc_Server* server);
} vnc_Server_AgentCallback;

/**
 * Creates a Server.  Note only one @ref vnc_Server should exist in your
 * application at a time.
 *
 * The Server captures the current session, which may be switched-out or
 * otherwise not visible on the console.
 *
 * @param agentPath The full path to the @c vncagent executable, or NULL to
 *     specify that @c vncagent is in the same directory as the app
 *     binary at run-time.  If you provide a directory, the executable must be
 *     called @c vncagent (@c vncagent.exe on Windows); if you have renamed
 *     @c vncagent then the filename must be provided.
 *
 * @return A new @ref vnc_Server or NULL on error, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @retval FileError
 *     @c vncagent could not be found, or the file specified does not exist
 * @retval AgentError
 *     @c vncagent cannot be executed or its version number does not match that
 *     of the SDK
 * @retval UnexpectedCall
 *     A @ref vnc_Server already exists in your application
 *
 * @see Use vnc_Server_createService() to create a Server which captures the
 *     console.
 */
VNC_SDK_API vnc_Server*
vnc_Server_create(_In_opt_z_ const char* agentPath);

/**
 * Creates a Server running as a service.  Note only one @ref vnc_Server should
 * exist in your application at a time.
 *
 * The Server captures the machine's console, which may be a logged-in user
 * session or a login prompt.  Running as a service requires elevated
 * privileges, and must be done from a Windows Service (Windows), a launch
 * daemon (Mac), or a root process (Linux).
 *
 * On Windows, the @c vncagent executable must be located within either the
 * <tt>\%ProgramFiles\%</tt> or <tt>\%ProgramFiles(x86)\%</tt> directories
 * in order to allow the injection of secure key sequences (e.g. Ctrl-Alt-Del).
 *
 * On Mac OS X, an instance of @c vncagent must separately be run in each
 * graphical session as a launch agent.  The @p agentPath provided here must
 * match the path of these launch agents in order that the SDK can communicate
 * with them.
 *
 * @param agentPath The path to the @c vncagent executable (see
 *     vnc_Server_create() for details).
 *
 * @return A new @ref vnc_Server or NULL on error, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @retval AgentError
 *     @c vncagent cannot be found or its version number does not match that
 *     of the SDK
 * @retval UnexpectedCall
 *     A @ref vnc_Server already exists in your application
 *
 * @see Use vnc_Server_create() to create a Server which captures the
 *     current session.
 *
 * @since 1.1
 */
VNC_SDK_API vnc_Server*
vnc_Server_createService(_In_opt_z_ const char* agentPath);

/**
 * Destroys the Server.
 *
 * Any Viewers still connected will be disconnected, and the peer's
 * @ref vnc_Viewer_CanReconnect flag will be set, indicating that server may
 * start again soon.  If this is not desired, call vnc_Server_disconnectAll()
 * without the @ref vnc_Server_DisconnectReconnect flag before destroying
 * the Server.
 *
 * @param server The current Server.
 */
VNC_SDK_API void
vnc_Server_destroy(vnc_Server* server);

/**
 * Sets connection-related callbacks for the Server.
 *
 * @param server The current Server.
 * @param callback The new connection event callback.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 * vnc_getLastError() to get the error code.
 */
VNC_SDK_API vnc_status_t
vnc_Server_setConnectionCallback(
    vnc_Server* server,
    _In_opt_ const vnc_Server_ConnectionCallback* callback,
    void* userData);

/**
 * Returns the total number of Viewers currently connected to the Server.
 *
 * @since 1.1
 */
VNC_SDK_API int
vnc_Server_getConnectionCount(vnc_Server* server);

/**
 * Sets security-related callbacks for the Server.
 *
 * @param server The current Server.
 * @param callback The new security event callback.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 * vnc_getLastError() to get the error code.
 */
VNC_SDK_API vnc_status_t
vnc_Server_setSecurityCallback(
    vnc_Server* server,
    _In_opt_ const vnc_Server_SecurityCallback* callback,
    void* userData);

/**
  * Changes permissions for a Viewer mid-session.
  *
  * @param server The current Server.
  * @param connection The Viewer whose permissions to change.
  * @param perms Any combination of the @ref vnc_Server_Permissions flags, or
  *     0 (no permissions) to render the session ineffective.
  *
  * @return #vnc_success or #vnc_failure, in which case call
  * vnc_getLastError() to get the error code.
  */
VNC_SDK_API vnc_status_t
vnc_Server_setPermissions(vnc_Server* server,
                          vnc_Connection* connection,
                          int perms);

/**
  * Gets the set of current permissions for a Viewer.
  *
  * @param server The current Server.
  * @param connection The Viewer whose permissions to get.
  *
  * @return Zero or more of the @ref vnc_Server_Permissions flags.
  */
VNC_SDK_API int
vnc_Server_getPermissions(vnc_Server* server, vnc_Connection* connection);

/**
 * Sets agent-related callbacks for the Server.
 *
 * These are not used on Android.
 *
 * @param server The current Server.
 * @param callback The new agent event callback.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @since 1.1
 */
VNC_SDK_API vnc_status_t
vnc_Server_setAgentCallback(
    vnc_Server* server,
    _In_opt_ const vnc_Server_AgentCallback* callback,
    void* userData);

/**
 * Determines if the @c vncagent process is ready and available to capture the
 * display and inject input events.
 *
 * If the @c vncagent is not ready, then the display cannot be captured at present.
 * This could be because it is starting up or being restarted due to a user
 * logout, or an error. On Linux, this can also indicate that the X server is
 * not running. On Android, @c vncagent is not used and this function will always
 * return #vnc_true.
 *
 * See vnc_Server_setAgentCallback() for receiving callbacks when the agent
 * process starts and stops.
 *
 * @param server The current Server.
 *
 * @retval #vnc_true
 *     The @c vncagent process is ready
 *
 * @retval #vnc_false
 *     The @c vncagent process is not ready or not running.
 *
 * @since 1.1
 */
VNC_SDK_API vnc_bool_t
vnc_Server_isAgentReady(vnc_Server* server);

/**
 * Specifies a friendly name for the Server, to send to connected Viewers.
 * By default, this is the device's hostname, plus the display number
 * where appropriate.
 *
 * @param server The current Server.
 * @param name The UTF-8-encoded friendly name.
 */
VNC_SDK_API void
vnc_Server_setFriendlyName(vnc_Server* server, const char* name);

/**
 * Obtains the Server's @ref vnc_ConnectionHandler for performing connection
 * operations.
 *
 * Typically, a @ref vnc_ConnectionHandler is passed to
 * vnc_CloudListener_create() to start the Server listening for connections.
 * However, you can reverse this methodology and pass it to
 * vnc_CloudConnector_create() instead in order to connect to listening Viewers
 * if you want (see Viewer.h for more information).
 *
 * @param server The current Server.
 *
 * @return A @ref vnc_ConnectionHandler, valid for the lifetime of the Server.
 */
VNC_SDK_API vnc_ConnectionHandler*
vnc_Server_getConnectionHandler(vnc_Server* server);

/**
 * Returns the address of a particular connected Viewer.
 *
 * The format of the null-terminated string is
 * <tt>[vnccloud:\<cloud-address\>]:\<id\></tt>
 * Note the string is only guaranteed to be valid until the next method call.
 *
 * @param server The current Server.
 * @param connection The connection whose Cloud address you want.  You can
 *     obtain a @ref vnc_Connection from your callback function
 *     vnc_Server_ConnectionCallback::connectionStarted.
 *
 * @return A NULL-terminated string in the format
 *     <tt>[vnccloud:\<cloud-address\>]:\<id\></tt>.
 */
VNC_SDK_API const char*
vnc_Server_getPeerAddress(vnc_Server* server,
                          vnc_Connection* connection);


/**
 * Enumeration of disconnection flags.
 *
 * @see vnc_Server_disconnect(), vnc_Server_disconnectAll()
 *
 * @since 1.1
 */
typedef enum {
  /**
   * The disconnection message is deemed to be sufficiently important that the
   * Viewer should alert the Viewer user.
   */
  vnc_Server_DisconnectAlert = 1,
  /**
   * Set the peer's @ref vnc_Viewer_CanReconnect flag, that is, the Server
   * informs the Viewer that the server may start again soon.
   */
  vnc_Server_DisconnectReconnect = 2,
} vnc_Server_DisconnectFlags;


/**
 * Disconnects a particular Viewer, optionally specifying a message.
 *
 * @param server The current Server.
 * @param connection The connection to terminate.  You can obtain a
 *     @ref vnc_Connection from your callback function
 *     vnc_Server_ConnectionCallback::connectionStarted.
 * @param message The message to send to the Viewer.
 * @param flags Zero or more of the @ref vnc_Server_DisconnectFlags flags.
 *
 * @see vnc_Viewer_getDisconnectMessage()
 */
VNC_SDK_API void
vnc_Server_disconnect(vnc_Server* server,
                      vnc_Connection* connection,
                      const char* message,
                      int flags);

/**
 * Disconnects all Viewers, optionally specifying a message.
 *
 * @param server The current Server.
 * @param message The message to send to all Viewers.
 * @param flags Zero or more of the @ref vnc_Server_DisconnectFlags flags.
 *
 * @see vnc_Viewer_getDisconnectMessage()
 */
VNC_SDK_API void
vnc_Server_disconnectAll(vnc_Server* server,
                         const char* message,
                         int flags);

/**
 * Specifies a blacklist threshold and timeout for the Server.  By default,
 * the threshold is 5, which means that a Viewer failing to authenticate five
 * times in a row is blacklisted for the default timeout of 10 seconds.  This
 * guards against brute-force and dictionary attacks.
 *
 * To completely disable blacklisting, set @p threshold and @p timeout to 0.
 * For security reasons this is not recommended.
 *
 * @param server The current Server.
 * @param threshold A number of unsuccessful authentication attempts before
 *     blacklisting.
 * @param timeout A number of seconds before a blacklisted Viewer can attempt
 *     to authenticate again.
 *
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 *     to get the error code.
 *
 * @retval InvalidArgument
 *     @p timeout is zero or negative
 */
VNC_SDK_API vnc_status_t
vnc_Server_setBlacklist(vnc_Server* server,
                        int threshold,
                        int timeout);

/**
 * Handles a Windows Service control event.  When Server has been created via
 * vnc_Server_createService(), then Windows Service control events must be
 * forwarded to the Server so that active session can be determined. See the
 * Win32 function
 * <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/ms685058(v=vs.85).aspx">RegisterServiceCtrlHandlerEx</a>
 * and the <a href="https://msdn.microsoft.com/en-us/library/windows/desktop/ms683241(v=vs.85).aspx">HandlerEx</a>
 * callback for more information.
 *
 * This function can be called for all notifications received via your @c HandlerEx
 * callback, but in particular it must be called for notifications where the
 * control code (@p dwControl) is @c SERVICE_CONTROL_SESSIONCHANGE.
 *
 * This method is designed to be called from the Service Control Dispatcher
 * thread, rather than the thread which called vnc_init().
 *
 * @param dwControl The control code passed to @c HandlerEx
 * @param dwEventType The type of event passed to @c HandlerEx
 * @param lpEventData The event data passed to @c HandlerEx
 *
 * @since 1.1
 */
VNC_SDK_API void
vnc_Server_serviceControlHandlerEx(vnc_Server* server, int dwControl,
                                   int dwEventType, void* lpEventData);
/**
 * Enables transmission of audio from the Server to the Viewer. The audio
 * stream will be automatically negotiated with the peer endpoint, if
 * possible, and any audio data will be captured from the device's default
 * audio mixer.
 * By default, this is enabled and the Server will attempt to negotiate an
 * audio channel with any connecting viewers.
 *
 * @param server The Server.
 * @param enable #vnc_true to enable audio playback, #vnc_false to disable.
 *
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 *     to get the error code.
 *
 * @retval PeerNotSupported
 *     The peer endpoint does not support audio
 * 
 * @retval NotSupported
 *     This platform does not support audio
 *
 * @since 1.9
 */
VNC_SDK_API vnc_status_t
vnc_Server_enableAudio(vnc_Server* server,
                       vnc_bool_t enable);

/**
 * Obtains the Server's @ref vnc_AnnotationManager for handling annotation
 * operations.
 *
 * @param server The current Server.
 *
 * @return A @ref vnc_AnnotationManager, valid for the lifetime of the Server.
 *
 * @since 1.4
 */
VNC_SDK_API vnc_AnnotationManager*
vnc_Server_getAnnotationManager(vnc_Server* server);

/**
 * Obtains the Server's @ref vnc_MessagingManager for handling messaging.
 *
 * @return A @ref vnc_MessagingManager or NULL on error, in which case
 *     call vnc_getLastError() to get the error code.
 *
 * @retval NotEnabled
 *     The Messaging feature is not enabled for this Server.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_MessagingManager*
vnc_Server_getMessagingManager(vnc_Server* server);

/**
 * Obtains the Server's @ref vnc_DisplayManager, for managing the list of
 * displays made available by the Server and for setting the Server's
 * capture method.
 *
 * @param server The current Server.
 * @return A @ref vnc_DisplayManager or NULL on error, in which case
 *     call vnc_getLastError() to get the error code.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_DisplayManager*
vnc_Server_getDisplayManager(vnc_Server* server);

/**
 * Enumeration of encryption levels.
 *
 * @see vnc_Server_setEncryptionLevel()
 */
typedef enum {
  /**
   * Use strong encryption.  This is the default.
   */
  vnc_Server_EncryptionLevel_Default = 0,
  /**
   * Use the strongest available encryption, possibly at the cost of speed.
   */
  vnc_Server_EncryptionLevel_Maximum,
  /**
   * Allow the negotation of encryption levels, including no encryption.
   * Use with caution.
   * Requires a Third-Party Connectivity Add-on.
   */
  vnc_Server_EncryptionLevel_AllowUnencrypted,
} vnc_Server_EncryptionLevel;

/**
 * Sets the desired encryption level of the session from the range of options
 * enumerated by @ref vnc_Server_EncryptionLevel.
 *
 * This will not affect the encryption level of established connections.
 *
 * If a Viewer and Server select different encryption levels, connections
 * between them take place at the higher of their selected levels.
 *
 * @param server The current Server.
 * @param level The new encryption level.
 *
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 *     to get the error code.
 *
 * @retval InvalidArgument
 *     The encryption level is invalid
 */
VNC_SDK_API vnc_status_t
vnc_Server_setEncryptionLevel(vnc_Server* server,
                              vnc_Server_EncryptionLevel level);

/**
 * Returns the encryption level being used with an incoming connection,
 * or the Server's current encryption level if the connection is NULL.
 *
 * @param server The current Server.
 * @param connection The incoming connection or NULL.
 */
VNC_SDK_API vnc_Server_EncryptionLevel
vnc_Server_getEncryptionLevel(vnc_Server* server,
                              vnc_Connection* connection);

/**
 * Set a password for use with third-party VNC Viewers, using VncAuth
 * authentication.
 * Use in conjunction with vnc_Server_setEncryptionLevel(), setting the level
 * to vnc_Server_EncryptionLevel_AllowUnencrypted, enabled by the Third-Party
 * Connectivity add-on.
 * If all requirements above are met, this password will be used instead of
 * the @ref vnc_Server_SecurityCallback callbacks.
 *
 * @param server The current Server.
 * @param password The VncAuth password.
 *
 * @retval UnexpectedCall
 *        The Third-Party Connectivity Add-on is not available
 *
 */
VNC_SDK_API vnc_status_t
vnc_Server_setVncAuthPassword(vnc_Server* server,
                              const char* password);


/**
 * Sets the number of seconds to wait before disconnecting idle Viewers.
 * To turn off the idle timeout, use a value of 0.
 *
 * @param server The current Server.
 * @param idleTimeout The idle timeout in seconds.
 *
 * @return #vnc_success or #vnc_failure, in which case call vnc_getLastError()
 *     to get the error code.
 *
 * @retval InvalidArgument
 *     The idle timeout is invalid
 */
VNC_SDK_API vnc_status_t
vnc_Server_setIdleTimeout(vnc_Server* server, int idleTimeout);

/**
 * Gets the current number of seconds to wait before disconnecting idle
 * Viewers. A value of 0 indicates no timeout is set.
 *
 * @param server The current Server.
 */
VNC_SDK_API int
vnc_Server_getIdleTimeout(vnc_Server* server);



/**
 * Sets input control related callbacks for the Server.
 *
 * @param server The current Server.
 * @param callback The new input event callbacks.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 * vnc_getLastError() to get the error code.
 */
VNC_SDK_API vnc_status_t
vnc_Server_setInputEventsCallback(
  vnc_Server* server,
  _In_opt_ const vnc_Server_InputEventsCallback* callback,
  void* userData);


/**
 * Specifies the screen capture method used by the Server.  By default, an
 * optimal method is chosen for each platform.  In the unlikely event you
 * experience screen capture issues under Windows or Linux, try the fallback
 * method (this is not supported under Mac OS X).
 *
 * @param server The current Server.
 * @param captureMethod One of the @ref vnc_Server_CaptureMethod flags.
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @retval NotSupported
 *     @p captureMethod is not supported on this platform.
 */
VNC_SDK_API vnc_status_t
vnc_Server_setCaptureMethod(vnc_Server* server,
                            vnc_Server_CaptureMethod captureMethod);

#ifdef __cplusplus
}
#endif

#endif
