/* Copyright (C) 2017-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_DIRECTUDP_H__
#define __VNC_DIRECTUDP_H__

#include "Common.h"

/** Listener used to receive incoming UDP connections */
typedef struct vnc_DirectUdpListener vnc_DirectUdpListener;
/** Connector used to make outgoing UDP connections */
typedef struct vnc_DirectUdpConnector vnc_DirectUdpConnector;

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Listens for or establishes connections using Direct UDP.
 *
 * @since 1.8 XXX
 */

/**
 * The default port for VNC direct UDP connections.
 */
#define VNC_DIRECT_UDP_DEFAULT_PORT 5900

/** Callback for filtering incoming UDP connections. */
typedef struct {
  /**
   * Notification to provide address-based filtering of incoming connections.
   * This callback is optional.
   *
   * @param listener The listening object that performed this callback.
   * @param ipAddress The IP address of the remote end that is attempting to
   *     make a connection. The address is presented in a human-readable form
   *     such as 122.16.224.1 (IPv4) and 2001:dc8:0:1534:0:867:6:1 (IPv6)
   * @param port The port on which a connection is being attempted.
   *
   * @return #vnc_true to allow the connection, or #vnc_false to deny, in which
   *     case the connection will be closed.
   *
   * @since 1.8 XXX
   */
  vnc_bool_t
  (*filterConnection)(void* userData,
                      vnc_DirectUdpListener* listener,
                      const char* ipAddress,
                      int port);
} vnc_DirectUdpListener_Callback;

/**
 * Begin listening for incoming UDP connections on the given port (IPv4 and
 * IPv6).  To stop listening, destroy the UDP listener.
 *
 * @param port The port number to listen on.
 * @param addressList A comma-separated list of addresses to listen on. If
 *     empty or null then listening starts on all addresses.
 * @param connectionHandler The object which shall handle incoming connections
 *     (if not rejected by the filter function).  It is either a server or a
 *     viewer.  It must be destroyed after the UDP listener.
 * @param callback An optional callback to filter incoming connections.
 * @param userData An optional pointer to user data.
 *
 * @return Returns a new UDP listener on success, or NULL in the case of an
 *     error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval AddressError
 *     There was an error listening on the specified address
 * @retval NotEnabled
 *     The SDK does not have the Direct UDP add-on enabled
 *
 * @since 1.8 XXX
 */
VNC_SDK_API vnc_DirectUdpListener*
vnc_DirectUdpListener_create(
                      int port,
                      _In_opt_z_ const char* addressList,
                      vnc_ConnectionHandler* connectionHandler,
                      _In_opt_ const vnc_DirectUdpListener_Callback* callback,
                      void* userData);

/**
 * Destroys the UDP listener.  Any ongoing connections are not affected, but
 * new connections will not be accepted.
 *
 * @param listener The UDP listener to destroy.
 *
 * @since 1.8 XXX
 */
VNC_SDK_API void
vnc_DirectUdpListener_destroy(vnc_DirectUdpListener* listener);


/**
 * Creates a new UDP Connector which is used to make outgoing connections
 *     to UDP listeners.
 *
 * @return Returns a new UDP Connector on success, or NULL in the case of an
 *     error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval NotEnabled
 *     The SDK does not have the Direct UDP add-on enabled
 *
 * @since 1.8 XXX
 */
VNC_SDK_API vnc_DirectUdpConnector*
vnc_DirectUdpConnector_create(void);

/**
 * Destroys the UDP Connector.

 * @param connector The UDP connector to destroy.
 *
 * @since 1.8 XXX
 */
VNC_SDK_API void
vnc_DirectUdpConnector_destroy(vnc_DirectUdpConnector* connector);

/**
 * Begins an outgoing UDP connection to the given hostname or IP address.  The
 * connection will be handled by the supplied @p connectionHandler.
 *
 * Connection errors are notified using the server or viewer's callbacks
 * (depending on the type of the connection handler used). This includes errors
 * arising when resolving a specified hostname.
 *
 * @param connector The UDP connector that is to begin the connection.
 * @param hostOrIpAddress The DNS hostname or IP address to connect to.
 * @param port The port number to connect to. Note: The default port for VNC
 *     connections can be obtained using @ref VNC_DIRECT_UDP_DEFAULT_PORT
 * @param connectionHandler The object (a Viewer or a Server) to handle the
 *     connection.
 *
 * @return #vnc_success is returned on success or #vnc_failure in the case of
 *     an error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval InvalidArgument
 *     @p port is invalid, or
 *     @p connectionHandler is a null pointer
 *
 * @since 1.8 XXX
 */
VNC_SDK_API vnc_status_t
vnc_DirectUdpConnector_connect(vnc_DirectUdpConnector* connector,
                               const char* hostOrIpAddress,
                               int port,
                               vnc_ConnectionHandler* connectionHandler);

#ifdef __cplusplus
}
#endif

#endif
