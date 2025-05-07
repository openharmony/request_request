/*
 * Copyright (c) 2025 Huawei Device Co., Ltd.
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

#ifndef ANIUTIL_MEMORY_H
#define ANIUTIL_MEMORY_H

#include <ani.h>
#include <refbase.h>

#include <memory>
#include <string>

namespace OHOS {
namespace AniUtil {

class NativeObject {
public:
    virtual ~NativeObject() = default;
};


template <typename T>
class StdSharedPtrHolder : public NativeObject {
public:
    StdSharedPtrHolder(const std::shared_ptr<T> &sptr) : sptr_(sptr)
    {
    }

    std::shared_ptr<T> Get()
    {
        return sptr_;
    }

    std::shared_ptr<T> GetOrDefault()
    {
        if (!sptr_) {
            sptr_ = std::make_shared<T>();
        }
        return sptr_;
    }

private:
    std::shared_ptr<T> sptr_;
};


template <typename T>
class OhSharedPtrHolder : public NativeObject {
public:
    OhSharedPtrHolder(const OHOS::sptr<T> &sptr) : sptr_(sptr)
    {
    }

    OHOS::sptr<T> Get()
    {
        return sptr_;
    }

    OHOS::sptr<T> GetOrDefault()
    {
        if (!sptr_) {
            sptr_ = OHOS::sptr<T>::MakeSptr();
        }
        return sptr_;
    }

private:
    OHOS::sptr<T> sptr_;
};


class NativePtrWrapper {
public:
    NativePtrWrapper(ani_env *env, ani_object object, const char* propName = "nativePtr")
        : env_(env), obj_(object), propName_(propName)
    {
    }

    template<typename T>
    ani_status Wrap(T* nativePtr)
    {
        return env_->Object_SetFieldByName_Long(obj_, propName_.c_str(), reinterpret_cast<ani_long>(nativePtr));
    }

    template<typename T>
    T* Unwrap()
    {
        ani_long nativePtr;
        if (ANI_OK != env_->Object_GetFieldByName_Long(obj_, propName_.c_str(), &nativePtr)) {
            return nullptr;
        }
        return reinterpret_cast<T*>(nativePtr);
    }

private:
    ani_env *env_ = nullptr;
    ani_object obj_ = nullptr;
    std::string propName_;
};


class NativePtrCleaner {
public:
    static void Clean([[maybe_unused]] ani_env *env, [[maybe_unused]] ani_object object)
    {
        ani_long ptr = 0;
        if (ANI_OK != env->Object_GetFieldByName_Long(object, "targetPtr", &ptr)) {
            return;
        }
        delete reinterpret_cast<NativeObject *>(ptr);
    }

    NativePtrCleaner(ani_env *env)
        : env_(env)
    {
    }

    ani_status Bind(ani_class cls)
    {
        std::array methods = {
            ani_native_function { "clean", nullptr, reinterpret_cast<void *>(NativePtrCleaner::Clean) },
        };

        if (ANI_OK != env_->Class_BindNativeMethods(cls, methods.data(), methods.size())) {
            return (ani_status)ANI_ERROR;
        };

        return ANI_OK;
    }

private:
    ani_env *env_ = nullptr;
};

} // namespace AniUtil
} // namespace OHOS

#endif
