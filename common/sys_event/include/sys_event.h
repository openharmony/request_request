/*
* Copyright (C) 2025 Huawei Device Co., Ltd.
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
*     http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
#ifndef SYS_EVENT_H
#define SYS_EVENT_H
#include <string>
#include <unordered_map>

#include "hisysevent.h"

namespace OHOS {
namespace Request {
//event
constexpr const char *STATISTIC_EVENT = "STATISTIC_EVENT";
constexpr const char *FAULT_EVENT = "FAULT_EVENT";

enum DfxErrorCode : uint32_t {
    APP_ERROR_00 = 0x00000000,
    APP_ERROR_01 = 0x00000001,
    APP_ERROR_02 = 0x00000002,
    INVALID_IPC_MESSAGE_A00 = 0x001FFFFF,
    INVALID_IPC_MESSAGE_A01 = 0x001F0000,
    INVALID_IPC_MESSAGE_A02 = 0x001F0001,
    INVALID_IPC_MESSAGE_A03 = 0x001F0100,
    INVALID_IPC_MESSAGE_A04 = 0x001F0101,
    INVALID_IPC_MESSAGE_A05 = 0x001F0200,
    INVALID_IPC_MESSAGE_A06 = 0x001F0201,
    INVALID_IPC_MESSAGE_A07 = 0x001F0300,
    INVALID_IPC_MESSAGE_A08 = 0x001F0301,
    INVALID_IPC_MESSAGE_A09 = 0x001F0400,
    INVALID_IPC_MESSAGE_A10 = 0x001F0401,
    INVALID_IPC_MESSAGE_A11 = 0x001F0500,
    INVALID_IPC_MESSAGE_A12 = 0x001F0501,
    INVALID_IPC_MESSAGE_A13 = 0x001F0600,
    INVALID_IPC_MESSAGE_A14 = 0x001F0601,
    INVALID_IPC_MESSAGE_A15 = 0x001F0700,
    INVALID_IPC_MESSAGE_A16 = 0x001F0701,
    INVALID_IPC_MESSAGE_A17 = 0x001F0800,
    INVALID_IPC_MESSAGE_A18 = 0x001F0801,
    INVALID_IPC_MESSAGE_A19 = 0x001F0900,
    INVALID_IPC_MESSAGE_A20 = 0x001F0901,
    INVALID_IPC_MESSAGE_A21 = 0x001F0A00,
    INVALID_IPC_MESSAGE_A22 = 0x001F0A01,
    INVALID_IPC_MESSAGE_A23 = 0x001F0B00,
    INVALID_IPC_MESSAGE_A24 = 0x001F0B01,
    INVALID_IPC_MESSAGE_A25 = 0x001F0C00,
    INVALID_IPC_MESSAGE_A26 = 0x001F0C01,
    INVALID_IPC_MESSAGE_A27 = 0x001F0D00,
    INVALID_IPC_MESSAGE_A28 = 0x001F0D01,
    INVALID_IPC_MESSAGE_A29 = 0x001F0E00,
    INVALID_IPC_MESSAGE_A30 = 0x001F0E01,
    INVALID_IPC_MESSAGE_A31 = 0x001F0F00,
    INVALID_IPC_MESSAGE_A32 = 0x001F0F01,
    INVALID_IPC_MESSAGE_A33 = 0x001F1000,
    INVALID_IPC_MESSAGE_A34 = 0x001F1001,
    INVALID_IPC_MESSAGE_A35 = 0x001F1100,
    INVALID_IPC_MESSAGE_A36 = 0x001F1101,
    INVALID_IPC_MESSAGE_A37 = 0x001F1200,
    INVALID_IPC_MESSAGE_A38 = 0x001F1201,
    INVALID_IPC_MESSAGE_A39 = 0x001F1300,
    INVALID_IPC_MESSAGE_A40 = 0x001F1301,
    INVALID_IPC_MESSAGE_A41 = 0x001F1400,
    INVALID_IPC_MESSAGE_A42 = 0x001F1401,
    INVALID_IPC_MESSAGE_A43 = 0x001F1500,
    INVALID_IPC_MESSAGE_A44 = 0x001F1501,
    INVALID_IPC_MESSAGE_A45 = 0x001F1600,
    INVALID_IPC_MESSAGE_A46 = 0x001F1601,
    INVALID_IPC_MESSAGE_00 = 0x00100000,
    INVALID_IPC_MESSAGE_01 = 0x00100100,
    INVALID_IPC_MESSAGE_02 = 0x00100200,
    INVALID_IPC_MESSAGE_03 = 0x00100300,
    INVALID_IPC_MESSAGE_04 = 0x00100400,
    INVALID_IPC_MESSAGE_05 = 0x00100500,
    INVALID_IPC_MESSAGE_06 = 0x00100600,
    INVALID_IPC_MESSAGE_07 = 0x00100700,
    INVALID_IPC_MESSAGE_08 = 0x00100800,
    INVALID_IPC_MESSAGE_09 = 0x00100900,
    INVALID_IPC_MESSAGE_10 = 0x00100A00,
    INVALID_IPC_MESSAGE_11 = 0x00100B00,
    INVALID_IPC_MESSAGE_12 = 0x00100C00,
    INVALID_IPC_MESSAGE_13 = 0x00100D00,
    INVALID_IPC_MESSAGE_14 = 0x00100E00,
    INVALID_IPC_MESSAGE_15 = 0x00100F00,
    INVALID_IPC_MESSAGE_16 = 0x00101000,
    INVALID_IPC_MESSAGE_17 = 0x00101100,
    INVALID_IPC_MESSAGE_18 = 0x00101200,
    INVALID_IPC_MESSAGE_19 = 0x00101300,
    INVALID_IPC_MESSAGE_20 = 0x00101400,
    INVALID_IPC_MESSAGE_21 = 0x00101500,
    INVALID_IPC_MESSAGE_22 = 0x00101600,
    TASK_FAULT_00 = 0x002F00FF,
    TASK_FAULT_01 = 0x002F01FF,
    TASK_FAULT_02 = 0x002F02FF,
    TASK_FAULT_03 = 0x002F03FF,
    TASK_FAULT_04 = 0x002F04FF,
    TASK_FAULT_05 = 0x002F05FF,
    TASK_FAULT_06 = 0x002F06FF,
    TASK_FAULT_07 = 0x002F07FF,
    TASK_FAULT_08 = 0x002F08FF,
    TASK_FAULT_09 = 0x002FFFFF,
    UDS_FAULT_00 = 0x00300000,
    UDS_FAULT_01 = 0x00300001,
    UDS_FAULT_02 = 0x00300002,
    UDS_FAULT_03 = 0x003F0000,
    UDS_FAULT_04 = 0x003F0001,
    SA_ERROR_00 = 0x004F0000,
    SA_ERROR_01 = 0x004F0001,
    SA_ERROR_02 = 0x004F0002,
    SA_FAULT_00 = 0x005F0000,
    SA_FAULT_01 = 0x005F0001,
    ACL_FAULT_00 = 0xF0000000,
    IPC_FAULT_00 = 0xF0100000,
    IPC_FAULT_01 = 0xF0100001,
    SAMGR_FAULT_00 = 0xF0200000,
    SAMGR_FAULT_01 = 0xF0200001,
    SAMGR_FAULT_02 = 0xF0200002,
    SAMGR_FAULT_03 = 0xF0200003,
    SAMGR_FAULT_04 = 0xF0200004,
    SAMGR_FAULT_A00 = 0xF02F0000,
    SAMGR_FAULT_A01 = 0xF02F0001,
    SAMGR_FAULT_A02 = 0xF02F0002,
    ABMS_FAULT_00 = 0xF0300000,
    ABMS_FAULT_01 = 0xF0300001,
    ABMS_FAULT_02 = 0xF0300002,
    ABMS_FAULT_03 = 0xF0300003,
    ABMS_FAULT_04 = 0xF0300004,
    ABMS_FAULT_05 = 0xF0300005,
    ABMS_FAULT_06 = 0xF0300006,
    ABMS_FAULT_07 = 0xF0300007,
    ABMS_FAULT_08 = 0xF0300008,
    ABMS_FAULT_09 = 0xF0300009,
    ABMS_FAULT_10 = 0xF030000A,
    ABMS_FAULT_11 = 0xF030000B,
    ABMS_FAULT_A00 = 0xF03F0000,
    ABMS_FAULT_A01 = 0xF03F0001,
    BMS_FAULT_00 = 0xF04F0000,
    OS_ACCOUNT_FAULT_00 = 0xF05F0000,
    OS_ACCOUNT_FAULT_01 = 0xF05F0001,
    OS_ACCOUNT_FAULT_02 = 0xF05F0002,
    RDB_FAULT_00 = 0xF06F0000,
    RDB_FAULT_01 = 0xF06F0001,
    RDB_FAULT_02 = 0xF06F0002,
    RDB_FAULT_03 = 0xF06F0003,
    RDB_FAULT_04 = 0xF06F0004,
    RDB_FAULT_05 = 0xF06F0005,
    RDB_FAULT_06 = 0xF06F0006,
    RDB_FAULT_07 = 0xF06F0007,
    RDB_FAULT_08 = 0xF06F0008,
    RDB_FAULT_09 = 0xF06F0009,
    RDB_FAULT_10 = 0xF06F000A,
    RDB_FAULT_11 = 0xF06F000B,
    RDB_FAULT_12 = 0xF06F000C,
    RDB_FAULT_13 = 0xF06FFFFF,
    EVENT_FAULT_00 = 0xF07F0000,
    EVENT_FAULT_01 = 0xF07F0001,
    EVENT_FAULT_02 = 0xF07F0002,
    NET_CONN_CLIENT_FAULT_00 = 0xF08F0000,
    NET_CONN_CLIENT_FAULT_01 = 0xF08F0001,
    NET_CONN_CLIENT_FAULT_02 = 0xF08F0002,
    NET_CONN_CLIENT_FAULT_03 = 0xF08F0003,
    TELEPHONY_FAULT_00 = 0xF09F0000,
    TELEPHONY_FAULT_01 = 0xF09F0001,
    SYSTEM_RESOURCE_FAULT_00 = 0xF0AF0000,
    SYSTEM_RESOURCE_FAULT_01 = 0xF0AF0001,
    SYSTEM_RESOURCE_FAULT_02 = 0xF0AF0002,
    MEDIA_FAULT_00 = 0xF0BF0000,
    MEDIA_FAULT_01 = 0xF0BF0001,
    NOTIFICATION_FAULT_00 = 0xF0CF0000,
    CERT_MANAGER_FAULT_00 = 0xF0DF0000,
    CERT_MANAGER_FAULT_01 = 0xF0DF0001,
    ACCESS_TOKEN_FAULT_00 = 0xF0EF0000,
    ACCESS_TOKEN_FAULT_01 = 0xF0EF0001,
    ACCESS_TOKEN_FAULT_02 = 0xF0EF0002,
    URL_POLICY_FAULT_00 = 0xF0FF0000,
    STANDARD_FAULT_00 = 0xF1000000,
    STANDARD_FAULT_01 = 0xF1000001,
    STANDARD_FAULT_02 = 0xF1000002,
    STANDARD_FAULT_03 = 0xF1000003,
    STANDARD_FAULT_04 = 0xF1000004,
    STANDARD_FAULT_05 = 0xF1000005,
    STANDARD_FAULT_06 = 0xF1000006,
    STANDARD_FAULT_A01 = 0xF10F0000,
};

struct SysEventInfo {
    uint32_t dCode;
    std::string bundleName;
    std::string moduleName;
    std::string extraInfo;
};

class SysEventLog {
public:
    static void SendSysEventLog(const std::string &eventName, const uint32_t dCode, const std::string bundleName,
        const std::string moduleName, const std::string extraInfo);
    static void SendSysEventLog(const std::string &eventName, const uint32_t dCode, const std::string extraInfo);
    static void SendSysEventLog(
        const std::string &eventName, const uint32_t dCode, const int32_t one, const int32_t two);

private:
    static std::unordered_map<std::string, void (*)(const SysEventInfo &info)> sysEventMap_;

    static void SendStatisticEvent(const SysEventInfo &info);
    static void SendFaultEvent(const SysEventInfo &info);

    template<typename... Types>
    static int32_t HisysWrite(const std::string &eventName, HiviewDFX::HiSysEvent::EventType type, Types... keyValues);
};

} // namespace Request
} // namespace OHOS

#endif