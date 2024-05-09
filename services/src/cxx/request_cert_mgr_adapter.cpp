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

#include "request_cert_mgr_adapter.h"

#include "log.h"

RequestCertManager &RequestCertManager::GetInstance()
{
    static RequestCertManager certManager;
    return certManager;
}

int32_t RequestCertManager::InitCertList(struct CertList **certList)
{
    *certList = static_cast<struct CertList *>(malloc(sizeof(struct CertList)));
    if (*certList == nullptr) {
        return CMR_ERROR_MALLOC_FAIL;
    }

    uint32_t buffSize = MAX_COUNT_CERTIFICATE * sizeof(struct CertAbstract);
    (*certList)->certAbstract = static_cast<struct CertAbstract *>(malloc(buffSize));
    if ((*certList)->certAbstract == nullptr) {
        free(*certList);
        *certList = nullptr;
        return CMR_ERROR_MALLOC_FAIL;
    }
    (void)memset_s((*certList)->certAbstract, buffSize, 0, buffSize);
    (*certList)->certsCount = MAX_COUNT_CERTIFICATE;

    return CM_SUCCESS;
}

int32_t RequestCertManager::InitCertInfo(struct CertInfo *certInfo)
{
    certInfo->certInfo.data = static_cast<uint8_t *>(malloc(MAX_LEN_CERTIFICATE));
    if (certInfo->certInfo.data == nullptr) {
        return CMR_ERROR_MALLOC_FAIL;
    }
    certInfo->certInfo.size = MAX_LEN_CERTIFICATE;

    return CM_SUCCESS;
}

void RequestCertManager::FreeCertList(CertList *certList)
{
    if (certList == nullptr) {
        return;
    }

    if (certList->certAbstract != nullptr) {
        free(certList->certAbstract);
        certList->certAbstract = nullptr;
    }

    free(certList);
    certList = nullptr;
}

void RequestCertManager::FreeCertData(struct CRequestCert *cert)
{
    if (cert == nullptr) {
        return;
    }

    if (cert->data != nullptr) {
        free(cert->data);
        cert->data = nullptr;
    }
    cert->size = 0;
    free(cert);
}

void RequestCertManager::FreeCertDataList(struct CRequestCerts *certs)
{
    for (uint32_t i = 0; i < certs->len; i++) {
        FreeCertData(certs->certDataList[i]);
    }
    free(certs->certDataList);
    free(certs);
}

void RequestCertManager::FreeCertInfo(struct CertInfo *certInfo)
{
    free(certInfo->certInfo.data);
    certInfo->certInfo.data = nullptr;
}

struct CRequestCerts *RequestCertManager::GetUserCertsData()
{
    struct CertList *certList = nullptr;
    int32_t ret = InitCertList(&certList);
    if (ret != CM_SUCCESS) {
        REQUEST_HILOGE("GetUserCertsData, init cert list failed, ret = %{public}d", ret);
        return nullptr;
    }

    ret = CmGetUserCertList(CM_USER_TRUSTED_STORE, certList);
    if (ret != CM_SUCCESS) {
        REQUEST_HILOGE("GetUserCertsData, get cert list failed, ret = %{public}d", ret);
        FreeCertList(certList);
        return nullptr;
    }

    struct CertInfo certInfo;
    struct CRequestCerts *certs = static_cast<struct CRequestCerts *>(malloc(sizeof(struct CRequestCerts)));
    if (certs == nullptr) {
        REQUEST_HILOGE("GetUserCertsData, malloc CRequestCerts failed");
        FreeCertList(certList);
        return nullptr;
    }
    certs->len = 0;
    struct CRequestCert **certDataList =
        static_cast<struct CRequestCert **>(malloc(MAX_COUNT_CERTIFICATE * sizeof(struct CRequestCert *)));
    if (certDataList == nullptr) {
        REQUEST_HILOGE("GetUserCertsData, malloc certDataList failed");
        free(certs);
        FreeCertList(certList);
        return nullptr;
    }
    certs->certDataList = certDataList;

    for (uint32_t i = 0; i < certList->certsCount; i++) {
        (void)memset_s(&certInfo, sizeof(struct CertInfo), 0, sizeof(struct CertInfo));
        ret = InitCertInfo(&certInfo);
        if (ret != CM_SUCCESS) {
            REQUEST_HILOGE("GetUserCertsData, init cert info failed, ret = %{public}d ", ret);
            FreeCertDataList(certs);
            FreeCertList(certList);
            return nullptr;
        }
        char *uri = certList->certAbstract[i].uri;
        struct CmBlob uriBlob = { strlen(uri) + 1, reinterpret_cast<uint8_t *>(uri) };

        ret = CmGetUserCertInfo(&uriBlob, CM_USER_TRUSTED_STORE, &certInfo);
        if (ret != CM_SUCCESS) {
            REQUEST_HILOGE("GetUserCertsData, CmGetUserCertInfo failed, ret = %{public}d", ret);
            FreeCertInfo(&certInfo);
            FreeCertDataList(certs);
            FreeCertList(certList);
            return nullptr;
        }

        struct CRequestCert *cert = static_cast<struct CRequestCert *>(malloc(sizeof(struct CRequestCert)));
        if (cert == nullptr) {
            FreeCertInfo(&certInfo);
            FreeCertDataList(certs);
            FreeCertList(certList);
            return nullptr;
        }
        cert->data = certInfo.certInfo.data;
        cert->size = certInfo.certInfo.size;
        certDataList[i] = cert;
        certs->len++;
    }
    FreeCertList(certList);
    return certs;
}

void FreeCertDataList(struct CRequestCerts *certs)
{
    RequestCertManager::GetInstance().FreeCertDataList(certs);
}

struct CRequestCerts *GetUserCertsData(void)
{
    return RequestCertManager::GetInstance().GetUserCertsData();
}