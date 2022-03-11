/* Copyright (C) 2016-2021 RealVNC Limited. All rights reserved.
 */

#ifndef __VNC_VNC_H__
#define __VNC_VNC_H__

/**
 * @file
 * Includes all the other VNC SDK headers.
 *
 * Note Server.h and Displays.h apply only to Servers, and Viewer.h and
 * Keyboard.h apply only to Viewers.  All other headers apply equally to
 * Servers and Viewers.
 */

/*
 * Headers for both Viewers and Servers:
 */
#include "AddOn.h"
#include "Annotation.h"
#include "Cloud.h"
#include "DataBuffer.h"
#include "DataStore.h"
#include "DirectTcp.h"
#include "DirectUdp.h"
#include "Errors.h"
#include "EventLoop.h"
#include "EventLoopFd.h"
#include "EventLoopWin.h"
#include "Init.h"
#include "Logger.h"
#include "Messaging.h"
#include "RsaKey.h"

/*
 * Headers just for Servers:
 */
#include "Server.h"
#include "Displays.h"

/*
 * Headers just for Viewers:
 */
#include "Viewer.h"
#include "Keyboard.h"

#endif
