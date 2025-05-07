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

#ifndef ANIUTIL_CLASS_H
#define ANIUTIL_CLASS_H

#include <ani.h>

#include <cstdarg>

#include "base.h"

namespace OHOS {
namespace AniUtil {

class TypeFinder {
public:
    TypeFinder(ani_env* env) : env_(env)
    {
    }

    expected<ani_namespace, ani_status> FindNamespace(const char* nsName)
    {
        ani_namespace ns;
        ani_status status = env_->FindNamespace(nsName, &ns);
        if (ANI_OK != status) {
            return status;
        }
        return ns;
    }

    template <typename... Names>
    expected<ani_namespace, ani_status> FindNamespace(const char* firstNs, const char* nextNs, Names... restNs)
    {
        ani_namespace ns;
        ani_status status = env_->FindNamespace(firstNs, &ns);
        if (ANI_OK != status) {
            return status;
        }
        return FindNamespace(ns, nextNs, restNs...);
    }

    expected<ani_class, ani_status> FindClass(const char* clsName)
    {
        ani_class cls;
        ani_status status = env_->FindClass(clsName, &cls);
        if (ANI_OK != status) {
            return status;
        }
        return cls;
    }

    expected<ani_class, ani_status> FindClass(const char* nsName, const char* clsName)
    {
        auto ns = FindNamespace(nsName, clsName);
        if (!ns.has_value()) {
            return ns.error();
        }
        return FindClass(ns.value(), clsName);
    }

    template <typename... Names>
    expected<ani_class, ani_status> FindClass(const char* firstNs, const char* secondNs,
        Names... restNs, const char* clsName)
    {
        auto ns = FindNamespace(firstNs, secondNs, restNs...);
        if (!ns.has_value()) {
            return ns.error();
        }
        return FindClass(ns.value(), clsName);
    }

    expected<ani_class, ani_status> FindClass(ani_namespace ns, const char* clsName)
    {
        ani_class cls;
        ani_status status = env_->Namespace_FindClass(ns, clsName, &cls);
        if (ANI_OK != status) {
            return status;
        }
        return cls;
    }

    expected<ani_enum, ani_status> FindEnum(ani_namespace ns, const char* enumName)
    {
        ani_enum aniEnum{};
        ani_status status = env_->Namespace_FindEnum(ns, enumName, &aniEnum);
        if (ANI_OK != status) {
            return status;
        }
        return aniEnum;
    }

private:
    template <typename... Names>
    expected<ani_namespace, ani_status> FindNamespace(ani_namespace currentNs, const char *nextNs, Names... restNs)
    {
        ani_namespace ns;
        ani_status status = env_->Namespace_FindNamespace(currentNs, nextNs, &ns);
        if (ANI_OK != status) {
            return status;
        }
        return FindNamespace(ns, restNs...);
    }

    expected<ani_namespace, ani_status> FindNamespace(ani_namespace currentNs)
    {
        return currentNs;
    }

private:
    ani_env* env_ = nullptr;
};


class ObjectFactory {
public:
    ObjectFactory(ani_env *env)
        : env_(env)
    {
    }

    expected<ani_object, ani_status> Create(const char* clsName, ...)
    {
        auto cls = TypeFinder(env_).FindClass(clsName);
        if (!cls.has_value()) {
            return cls.error();
        }

        va_list args;
        va_start(args, clsName);
        auto obj = CreateV(cls.value(), args);
        va_end(args);
        return obj;
    }

    template<typename... Names>
    expected<ani_object, ani_status> Create(const char* nsName, Names... restNs, const char* clsName, ...)
    {
        auto cls = TypeFinder(env_).FindClass(nsName, restNs..., clsName);
        if (!cls.has_value()) {
            return cls.error();
        }

        va_list args;
        va_start(args, clsName);
        auto obj = CreateV(cls.value(), args);
        va_end(args);
        return obj;
    }

    expected<ani_object, ani_status> Create(ani_class cls, ...)
    {
        va_list args;
        va_start(args, cls);
        auto obj = CreateV(cls, args);
        va_end(args);
        return obj;
    }

private:
    expected<ani_object, ani_status> CreateV(ani_class cls, va_list args)
    {
        ani_method ctor;
        ani_status status = env_->Class_FindMethod(cls, "<ctor>", nullptr, &ctor);
        if (ANI_OK != status) {
            return status;
        }

        ani_object obj;
        status = env_->Object_New_V(cls, ctor, &obj, args);
        if (ANI_OK != status) {
            return status;
        }
        return obj;
    }

private:
    ani_env *env_ = nullptr;
};

} // namespace AniUtil
} // namespace OHOS

#endif