/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_DIRECTTCP_H__
#define __VNC_DIRECTTCP_H__

#include "Common.h"

/** Listener used to receive incoming TCP connections */
typedef struct vnc_DirectTcpListener vnc_DirectTcpListener;
/** Connector used to make outgoing TCP connections */
typedef struct vnc_DirectTcpConnector vnc_DirectTcpConnector;

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Listens for or establishes connections using Direct TCP.
 *
 * @since 1.2
 */

/**
 * The default port for VNC direct TCP connections.
 */
#define VNC_DIRECT_TCP_DEFAULT_PORT 5900

/** Callback for filtering incoming TCP connections. */
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
   * @since 1.2
   */
  vnc_bool_t
  (*filterConnection)(void* userData,
                      vnc_DirectTcpListener* listener,
                      const char* ipAddress,
                      int port);
} vnc_DirectTcpListener_Callback;

/**
 * Begin listening for incoming TCP connections on the given port (IPv4 and
 * IPv6).  To stop listening, destroy the TCP listener.
 *
 * @param port The port number to listen on.
 * @param addressList A comma-separated list of addresses to listen on. If
 *     empty or null then listening starts on all addresses.
 * @param connectionHandler The object which shall handle incoming connections
 *     (if not rejected by the filter function).  It is either a server or a
 *     viewer.  It must be destroyed after the TCP listener.
 * @param callback An optional callback to filter incoming connections.
 * @param userData An optional pointer to user data.
 *
 * @return Returns a new TCP listener on success, or NULL in the case of an
 *     error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval AddressError
 *     There was an error listening on the specified address
 * @retval NotEnabled
 *     The SDK does not have the Direct TCP add-on enabled
 *
 * @since 1.2
 */
VNC_SDK_API vnc_DirectTcpListener*
vnc_DirectTcpListener_create(
                      int port,
                      _In_opt_z_ const char* addressList,
                      vnc_ConnectionHandler* connectionHandler,
                      _In_opt_ const vnc_DirectTcpListener_Callback* callback,
                      void* userData);

/**
 * Destroys the TCP listener.  Any ongoing connections are not affected, but
 * new connections will not be accepted.
 *
 * @param listener The TCP listener to destroy.
 *
 * @since 1.2
 */
VNC_SDK_API void
vnc_DirectTcpListener_destroy(vnc_DirectTcpListener* listener);


/**
 * Creates a new TCP Connector which is used to make outgoing connections
 *     to TCP listeners.
 *
 * @return Returns a new TCP Connector on success, or NULL in the case of an
 *     error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval NotEnabled
 *     The SDK does not have the Direct TCP add-on enabled
 *
 * @since 1.2
 */
VNC_SDK_API vnc_DirectTcpConnector*
vnc_DirectTcpConnector_create(void);

/**
 * Set proxy server settings for this TCP Connector. The settings are adopted
 * for all subsequent outgoing connections for this connector.
 * The proxy settings can be either the system configured settings or as
 * specified in a supplied settings string.
 * If the proxy server requires authentication then full user credentials
 * (i.e. username and password) must be configured.
 * Any connection error arising from the proxy settings is reported when an
 * outgoing connection is attempted, see @ref vnc_DirectTcpConnector_connect
 *
 * @param connector The TCP connector that will employ the proxy settings
 * @param systemProxy Set this to #vnc_true to employ the currently
 *     configured system proxy settings. When set to #vnc_false, @p proxyUrl is
 *     used to specify the proxy settings.
 * @param proxyUrl A string specifying a proxy server, plus user credentials
 *     where these are required by an authenticated proxy server.
 *     The format of the string is:  type://username:password@address::port
 *     Type is either 'socks' or 'http'
 *     Example: socks://fred:X$fr£4Pw@mySocks5Proxy.here.com::8888
 *     If @p systemProxy is #vnc_false and the proxyUrl string is null or empty
 *     then this is treated as a request to stop using a proxy server.
 *
 * @return #vnc_success is returned on success or #vnc_failure in the case of
 *     an error, in which case vnc_getLastError() can be used to get the error
 *     code.
 *
 * @retval InvalidArgument
 *     @p proxyUrl does not conform to the required format (see above)
 *
 * @since 1.2
 */
VNC_SDK_API vnc_status_t
vnc_DirectTcpConnector_setProxySettings(vnc_DirectTcpConnector* connector,
                                        vnc_bool_t systemProxy,
                                        _In_opt_z_ const char* proxyUrl);

/**
 * Destroys the TCP Connector.

 * @param connector The TCP connector to destroy.
 *
 * @since 1.2
 */
VNC_SDK_API void
vnc_DirectTcpConnector_destroy(vnc_DirectTcpConnector* connector);

/**
 * Begins an outgoing TCP connection to the given hostname or IP address.  The
 * connection will be handled by the supplied @p connectionHandler.
 *
 * The connection shall be via direct TCP unless a preceding call was made to
 * @ref vnc_DirectTcpConnector_setProxySettings to result in connections being
 * made via a proxy server.
 *
 * Connection errors are notified using the server or viewer's callbacks
 * (depending on the type of the connection handler used). This includes errors
 * arising when resolving a specified hostname.
 *
 * @param connector The TCP connector that is to begin the connection.
 * @param hostOrIpAddress The DNS hostname or IP address to connect to.
 * @param port The port number to connect to. Note: The default port for VNC
 *     connections can be obtained using @ref VNC_DIRECT_TCP_DEFAULT_PORT
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
 * @since 1.2
 */
VNC_SDK_API vnc_status_t
vnc_DirectTcpConnector_connect(vnc_DirectTcpConnector* connector,
                               const char* hostOrIpAddress,
                               int port,
                               vnc_ConnectionHandler* connectionHandler);

#ifdef __cplusplus
}
#endif

#endif
