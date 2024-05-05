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

#ifndef C_FORM_ITEM_H
#define C_FORM_ITEM_H

#include <string>

#include "c_string_wrapper.h"

struct CFileSpec {
    CStringWrapper name;
    CStringWrapper path;
    CStringWrapper fileName;
    CStringWrapper mimeType;
    bool is_user_file;
};

struct FileSpec {
    std::string name;
    std::string path;
    std::string fileName;
    std::string mimeType;
    bool is_user_file;
};

struct CFormItem {
    CStringWrapper name;
    CStringWrapper value;
};

struct FormItem {
    std::string name;
    std::string value;
};

#endif // C_FORM_ITEM_H