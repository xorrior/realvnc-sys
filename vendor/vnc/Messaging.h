/* Copyright (C) 2017-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_MESSAGING_H__
#define __VNC_MESSAGING_H__

#include "Common.h"
#ifdef __cplusplus
extern "C" {
#endif

/**
 * @file
 * Enables a Viewer and Server to send and receive messages over the same
 * secure data channel as the current screen sharing session.
 *
 * To enable a Viewer app to send and/or receive messages, call
 * @ref vnc_Viewer_getMessagingManager. To enable a Server app to send
 * and/or receive messages, call @ref vnc_Server_getMessagingManager. Both
 * functions return a @ref vnc_MessagingManager object.
 *
 * Call @ref vnc_MessagingManager_sendMessage to send a message. To be
 * notified when a message is received, call
 * @ref vnc_MessagingManager_setCallback and monitor
 * @ref vnc_MessagingManager_Callback::messageReceived.
 *
 * You can send up to 1MB of data per message, in any format (note the
 * Python sample apps demonstrating this feature send JSON-RPC). The
 * vnc_DataBuffer object received is owned by the SDK and destroyed
 * as soon as the callback completes. You should take a copy of this
 * data to interact with it, and do not call @ref vnc_DataBuffer_destroy
 * on the object.
 *
 * If sending a message reports #vnc_success, the message is
 * guaranteed to be sent in sequence order. However, the sender is not
 * informed by the underlying transport mechanism *when* it has been sent,
 * and nor is it notified when a message has been received. You should
 * send your own ACK packets to implement this behavior.
 * If a disconnect occurs after @ref vnc_MessagingManager_sendMessage is called and
 * before the data has been sent there is a guarantee that the data will be dropped
 * and a send will not be attempted on the reconnection.
 *
 * If sending a message reports #vnc_failure, no message will be sent.
 * Partial messages are not sent.
 *
 * @since 1.5
 */


/**
 * Callback notifying when messages are received.
 *
 * @see vnc_MessagingManager_setCallback
 */
typedef struct {
  /**
   * Notification that a message has been received.
   *
   * The vnc_Databuffer object containing the data is owned by the SDK
   * and is automatically destroyed when the callback completes. Take a copy
   * of the data to interact with it. Do not call @ref vnc_DataBuffer_destroy.
   *
   * @param messagingManager Belonging to either a Viewer or a Server.
   * @param sender The peer sending the message.
   * @param buffer The data received.
   *
   * @since 1.5
   */
  void
  (*messageReceived)(void* userData,
                     vnc_MessagingManager* messagingManager,
                     vnc_Connection* sender,
                     const vnc_DataBuffer* buffer);

} vnc_MessagingManager_Callback;


/**
 * Registers a callback notifying when messages are received.
 *
 * You should call this function before your app attempts
 * either to connect or to listen for connections.
 *
 * @param messagingManager Belonging to either a Viewer or a Server.
 * @param callback The callback.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_status_t
vnc_MessagingManager_setCallback(
    vnc_MessagingManager* messagingManager,
    _In_opt_ const vnc_MessagingManager_Callback* callback,
    void* userData);

/**
 * Sends a message.
 *
 * @param messagingManager Belonging to either a Viewer or a Server.
 * @param buffer The data to send.
 * @param bufferLength The number of bytes. Maximum 1MB.
 * @param connection For a Server, the Viewer to send the message to. To send
 * to all Viewers, iterate over connections. For a Viewer, specify NULL.
 *
 * @return #vnc_success or #vnc_failure, in which case call
 *     vnc_getLastError() to get the error code.
 *
 * @retval PeerNotSupported
 *     The peer does not support receiving messages (perhaps no add-on
 *     code has been applied).
 * @retval BufferFull
 *     Too much data is being sent.
 *
 * @since 1.5
 */
VNC_SDK_API vnc_status_t
vnc_MessagingManager_sendMessage(vnc_MessagingManager* messagingManager,
                                 _In_reads_bytes_(bufferLength) const void* buffer,
                                 int bufferLength,
                                 _In_opt_ vnc_Connection* connection);

#ifdef __cplusplus
}
#endif

#endif
