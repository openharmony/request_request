/*
 * Copyright (C) 2023 Huawei Device Co., Ltd.
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

#ifndef REQUEST_CERT_MGR_ADAPTER
#define REQUEST_CERT_MGR_ADAPTER

#include <securec.h>
#include <stdint.h>

#include <cstdint>
#include <vector>

#include "cert_manager_api.h"
#include "cm_type.h"

struct CRequestCert {
    uint32_t size;
    uint8_t *data;
};

struct CRequestCerts {
    struct CRequestCert **certDataList;
    uint32_t len;
};

class RequestCertManager {
public:
    static RequestCertManager &GetInstance();
    void FreeCertDataList(struct CRequestCerts *certs);
    struct CRequestCerts *GetUserCertsData();

private:
    int32_t InitCertList(struct CertList **certList);
    int32_t InitCertInfo(struct CertInfo *certInfo);
    void FreeCertList(CertList *certList);
    void FreeCertData(struct CRequestCert *cert);
    void FreeCertInfo(struct CertInfo *certInfo);
};

#ifdef __cplusplus
extern "C" {
#endif

void FreeCertDataList(struct CRequestCerts *certs);
struct CRequestCerts *GetUserCertsData(void);

#ifdef __cplusplus
}
#endif
#endif // REQUEST_CERT_MGR_ADAPTER