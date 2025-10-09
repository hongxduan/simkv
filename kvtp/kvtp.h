//
// Created by HongXing on 4/10/25.
//

#ifndef SIMKV_KVTP_H
#define SIMKV_KVTP_H

#define ZERO '\0'
#define COLON ':'
#define SPACE ' '
#define SPACE_STR " "
#define LINE_FEED '\n'

#define PROTOCOL_V1 "KVTP/1"
#define KEY_PREFIX "KEY"
#define CMD_PREFIX "CMD"
#define ARGS_PREFIX "ARGS"
#define TTL_PREFIX "TTL"

#define ARG_EX  "-EX"   // Exist
#define ARG_NX  "-NX"   // Not Exist
#define ARG_DEL  "-DEL" // Delete
#define ARG_TTL  "-TTL" // Ttl


#define RESP_STATUS_OK "OK"
#define RESP_STATUS_ERR "ERR"

#endif //SIMKV_KVTP_H
