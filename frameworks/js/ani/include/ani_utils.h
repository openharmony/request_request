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

#ifndef ANI_UTILS_H
#define ANI_UTILS_H

#include <ani.h>

#include <cstdarg>
#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <vector>
#include "log.h"
#include "base.h"

namespace OHOS {
namespace AniUtil {

class AniObjectUtils {
public:
    static ani_object Create(ani_env *env, const char* nsName, const char* clsName, ...)
    {
        ani_class cls;
        const std::string fullClassName = std::string(nsName).append(".").append(clsName);
        if (ANI_OK != env->FindClass(fullClassName.c_str(), &cls)) {
            REQUEST_HILOGE("[ANI] Not found namespace %{public}s", fullClassName);
            return nullptr;
        }

        ani_object obj;
        va_list args;
        va_start(args, clsName);
        obj = CreateV(env, cls, args);
        va_end(args);
        return obj;
    }

    static ani_object Create(ani_env *env, const char* nsName, const char* subNsName, const char* clsName, ...)
    {
        ani_class cls;
        const std::string fullClassName =
            std::string(nsName).append(".").append(subNsName).append(".").append(className);
        if (ANI_OK != env->FindClass(fullClassName.c_str(), &cls)) {
            REQUEST_HILOGE("[ANI] Not found class %{public}s", fullClassName);
            return nullptr;
        }

        ani_object obj;
        va_list args;
        va_start(args, clsName);
        obj = CreateV(env, cls, args);
        va_end(args);
        return obj;
    }

    static ani_object Create(ani_env *env, const char* clsName, ...)
    {
        ani_class cls;
        if (ANI_OK != env->FindClass(clsName, &cls)) {
            REQUEST_HILOGE("[ANI] Not found class %{public}s", clsName);
            return nullptr;
        }

        ani_object obj;
        va_list args;
        va_start(args, clsName);
        obj = CreateV(env, cls, args);
        va_end(args);
        return obj;
    }

    static ani_object Create(ani_env *env, ani_class cls, ...)
    {
        ani_method ctor;
        if (ANI_OK != env->Class_FindMethod(cls, "<ctor>", nullptr, &ctor)) {
            REQUEST_HILOGE("[ANI] Not found <ctor> for class");
            return nullptr;
        }

        ani_object obj;
        va_list args;
        va_start(args, cls);
        obj = CreateV(env, cls, args);
        va_end(args);
        return obj;
    }

    static ani_object From(ani_env *env, bool value)
    {
        return Create(env, "std.core.Boolean", static_cast<ani_boolean>(value));
    }

    template<typename T>
    static ani_status Wrap(ani_env *env, ani_object object, T* nativePtr, const char* propName = "nativePtr")
    {
        return env->Object_SetFieldByName_Long(object, propName, reinterpret_cast<ani_long>(nativePtr));
    }

    template<typename T>
    static T* Unwrap(ani_env *env, ani_object object, const char* propName = "nativePtr")
    {
        ani_long nativePtr;
        if (ANI_OK != env->Object_GetFieldByName_Long(object, propName, &nativePtr)) {
            return nullptr;
        }
        return reinterpret_cast<T*>(nativePtr);
    }

private:
    static ani_object CreateV(ani_env *env, ani_class cls, va_list args)
    {
        ani_method ctor;
        if (ANI_OK != env->Class_FindMethod(cls, "<ctor>", nullptr, &ctor)) {
            REQUEST_HILOGE("[ANI] Not found <ctor> for class");
            return nullptr;
        }

        ani_object obj;
        ani_status status = env->Object_New_V(cls, ctor, &obj, args);
        if (ANI_OK != status) {
            REQUEST_HILOGE("[ANI] Failed to Object_New for class.");
            return nullptr;
        }
        return obj;
    }
};


class AniStringUtils {
public:
    static std::string ToStd(ani_env *env, ani_string ani_str)
    {
        ani_size strSize;
        env->String_GetUTF8Size(ani_str, &strSize);

        std::vector<char> buffer(strSize + 1); // +1 for null terminator
        char* utf8_buffer = buffer.data();

        //String_GetUTF8 Supportted by https://gitee.com/openharmony/arkcompiler_runtime_core/pulls/3416
        ani_size bytes_written = 0;
        env->String_GetUTF8(ani_str, utf8_buffer, strSize + 1, &bytes_written);

        utf8_buffer[bytes_written] = '\0';
        std::string content = std::string(utf8_buffer);
        return content;
    }

    static ani_string ToAni(ani_env* env, const std::string& str)
    {
        ani_string aniStr = nullptr;
        if (ANI_OK != env->String_NewUTF8(str.data(), str.size(), &aniStr)) {
            REQUEST_HILOGE("[ANI] Unsupported ANI_VERSION_1");
            return nullptr;
        }
        return aniStr;
    }
};


class UnionAccessor {
public:
    UnionAccessor(ani_env *env, ani_object obj) : env_(env), obj_(obj)
    {
    }

    bool IsInstanceOf(const std::string& cls_name)
    {
        ani_class cls;
        env_->FindClass(cls_name.c_str(), &cls);

        ani_boolean ret;
        env_->Object_InstanceOf(obj_, cls, &ret);
        return ret;
    }

    template<typename T>
    bool IsInstanceOfType();

    template<typename T>
    expected<T, ani_status> Convert()
    {
        T value{};
        bool status = TryConvert<T>(value);
        if (ANI_OK != status) {
            return ANI_ERROR;
        }
        return value;
    }

    template<typename T>
    bool TryConvert(T &value);

    template<typename T>
    bool TryConvertArray(std::vector<T> &value);

private:
    ani_env *env_;
    ani_object obj_;
};

template<>
inline bool UnionAccessor::IsInstanceOfType<bool>()
{
    return IsInstanceOf("Lstd/core/Boolean;");
}

template<>
inline bool UnionAccessor::IsInstanceOfType<int>()
{
    return IsInstanceOf("Lstd/core/Int;");
}

template<>
inline bool UnionAccessor::IsInstanceOfType<double>()
{
    return IsInstanceOf("Lstd/core/Double;");
}

template<>
inline bool UnionAccessor::IsInstanceOfType<std::string>()
{
    return IsInstanceOf("Lstd/core/String;");
}

template<>
inline bool UnionAccessor::TryConvert<bool>(bool &value)
{
    if (!IsInstanceOfType<bool>()) {
        return false;
    }

    ani_boolean aniValue;
    auto ret = env_->Object_CallMethodByName_Boolean(obj_, "unboxed", nullptr, &aniValue);
    if (ret != ANI_OK) {
        return false;
    }
    value = static_cast<bool>(aniValue);
    return true;
}

template<>
inline bool UnionAccessor::TryConvert<int>(int &value)
{
    if (!IsInstanceOfType<int>()) {
        return false;
    }

    ani_int aniValue;
    auto ret = env_->Object_CallMethodByName_Int(obj_, "unboxed", nullptr, &aniValue);
    if (ret != ANI_OK) {
        return false;
    }
    value = static_cast<int>(aniValue);
    return true;
}

template<>
inline bool UnionAccessor::TryConvert<double>(double &value)
{
    if (!IsInstanceOfType<double>()) {
        return false;
    }

    ani_double aniValue;
    auto ret = env_->Object_CallMethodByName_Double(obj_, "unboxed", nullptr, &aniValue);
    if (ret != ANI_OK) {
        return false;
    }
    value = static_cast<double>(aniValue);
    return true;
}

template<>
inline bool UnionAccessor::TryConvert<std::string>(std::string &value)
{
    if (!IsInstanceOfType<std::string>()) {
        return false;
    }

    value = AniStringUtils::ToStd(env_, static_cast<ani_string>(obj_));
    return true;
}

ani_boolean IsInstanceOf(ani_env *env, const std::string &cls_name, ani_object obj);

class OptionalAccessor {
public:
    OptionalAccessor(ani_env *env, ani_object obj) : env_(env), obj_(obj)
    {
    }

    bool IsUndefined()
    {
        ani_boolean isUndefined;
        env_->Reference_IsUndefined(obj_, &isUndefined);
        return isUndefined;
    }

    template<typename T>
    expected<T, ani_status> Convert();

private:
    expected<std::string, ani_status> ConvertToString()
    {
        if (IsUndefined()) {
            return ANI_ERROR;
        }

        ani_size strSize;
        env_->String_GetUTF8Size(static_cast<ani_string>(obj_), &strSize);

        std::vector<char> buffer(strSize + 1);
        char* utf8_buffer = buffer.data();

        ani_size bytes_written = 0;
        env_->String_GetUTF8(static_cast<ani_string>(obj_), utf8_buffer, strSize + 1, &bytes_written);

        utf8_buffer[bytes_written] = '\0';
        std::string content = std::string(utf8_buffer);
        return content;
    }

private:
    ani_env *env_;
    ani_object obj_;
};

template<>
inline expected<bool, ani_status> OptionalAccessor::Convert<bool>()
{
    if (IsUndefined()) {
        return ANI_ERROR;
    }

    ani_boolean aniValue;
    auto ret = env_->Object_CallMethodByName_Boolean(obj_, "unboxed", nullptr, &aniValue);
    if (ret != ANI_OK) {
        return ret;
    }
    auto value = static_cast<bool>(aniValue);
    return value;
}

template<>
inline expected<double, ani_status> OptionalAccessor::Convert<double>()
{
    if (IsUndefined()) {
        return ANI_ERROR;
    }

    ani_double aniValue;
    auto ret = env_->Object_CallMethodByName_Double(obj_, "doubleValue", nullptr, &aniValue);
    if (ret != ANI_OK) {
        return ret;
    }
    auto value = static_cast<double>(aniValue);
    return value;
}

template<>
inline expected<std::string, ani_status> OptionalAccessor::Convert<std::string>()
{
    return ConvertToString();
}


class EnumAccessor {
public:
    EnumAccessor(ani_env *env, const char* className, ani_int index) : env_(env)
    {
        initStatus_ = ANI_ERROR;
        ani_enum_item item;
        initStatus_ = GetItem(className, index, item);
        if (ANI_OK == initStatus_) {
            item_ = item;
        }
    }

    EnumAccessor(ani_env *env, ani_enum_item item) : env_(env), item_(item)
    {
        initStatus_ = ANI_ERROR;
    }

    template<typename T>
    expected<T, ani_status> To()
    {
        int32_t value{};
        ani_status status = ToInt(value);
        if (ANI_OK != status) {
            return status;
        }
        return static_cast<T>(value);
    }

    ani_status ToInt(int32_t &value)
    {
        if (!item_) {
            return initStatus_;
        }

        ani_status status = env_->EnumItem_GetValue_Int(item_.value(), &value);
        if (ANI_OK != status) {
            REQUEST_HILOGE("Failed to call EnumItem_GetValue_Int");
            return status;
        }
        return ANI_OK;
    }

    expected<int32_t, ani_status> ToInt()
    {
        int32_t value;
        ani_status status = ToInt(value);
        if (ANI_OK != status) {
            return status;
        }
        return value;
    }

    ani_status ToString(std::string &value)
    {
        if (!item_) {
            return initStatus_;
        }

        ani_string strValue;
        ani_status status = env_->EnumItem_GetValue_String(item_.value(), &strValue);
        if (ANI_OK != status) {
            REQUEST_HILOGE("Failed to call EnumItem_GetValue_String");
            return status;
        }
        value = AniStringUtils::ToStd(env_, strValue);
        return ANI_OK;
    }

    expected<std::string, ani_status> ToString()
    {
        std::string value;
        ani_status status = ToString(value);
        if (ANI_OK != status) {
            return status;
        }
        return value;
    }

private:
    ani_status GetItem(const char* className, ani_int index, ani_enum_item &item)
    {
        ani_status status = ANI_ERROR;
        ani_enum enumType;
        status = env_->FindEnum(className, &enumType);
        if (ANI_OK != status) {
            REQUEST_HILOGE("Failed to call FindEnum for %{public}s", className);
            return status;
        }

        status = env_->Enum_GetEnumItemByIndex(enumType, index, &item);
        if (ANI_OK != status) {
            REQUEST_HILOGE("Failed to call Enum_GetEnumItemByIndex for %{public}s, [%{public}d]", className, index);
            return status;
        }
        return ANI_OK;
    }

private:
    ani_env *env_;
    std::optional<ani_enum_item> item_;
    ani_status initStatus_;
};


class ArrayAccessor {
public:
    ArrayAccessor(ani_env *env, ani_object obj) : env_(env), obj_(obj)
    {
    }

    ani_status Length(std::size_t &length)
    {
        ani_double value;
        if (ANI_OK != env_->Object_GetPropertyByName_Double(obj_, "length", &value)) {
            return ANI_ERROR;
        }
        length = static_cast<std::size_t>(value);
        return ANI_OK;
    }

    template <typename OutputIterator, typename TransformFunc>
    ani_status Transform(OutputIterator out, TransformFunc&& transform)
    {
        ani_status status = ANI_ERROR;
        std::size_t length = 0;
        status = Length(length);
        if (ANI_OK != status) {
            return status;
        }
        for (std::size_t i = 0; i < length; i++) {
            ani_ref itemRef;
            status = env_->Object_CallMethodByName_Ref(obj_, "$_get", "I:Lstd/core/Object;", &itemRef, (ani_int)i);
            if (ANI_OK != status) {
                return status;
            }
            typename OutputIterator::container_type::value_type value;
            status = transform(env_, itemRef, value);
            if (ANI_OK != status) {
                return status;
            }
            *out++ = value;
        }
        return ANI_OK;
    }

private:
    ani_env *env_ = nullptr;
    ani_object obj_ = nullptr;
};

struct ToDouble {
    ani_status operator()(ani_env *env, ani_ref &itemRef, double &value) const
    {
        ani_double aniValue;
        ani_object itemObj = static_cast<ani_object>(itemRef);
        ani_status status = env->Object_CallMethodByName_Double(itemObj, "unboxed", nullptr, &aniValue);
        if (ANI_OK != status) {
            value = static_cast<double>(aniValue);
        }
        return status;
    }
};


class AniLocalScopeGuard {
public:
    AniLocalScopeGuard(ani_env *env, size_t nrRefs) : env_(env)
    {
        status_ = env_->CreateLocalScope(nrRefs);
    }

    ~AniLocalScopeGuard()
    {
        if (ANI_OK != status_) {
            return;
        }
        env_->DestroyLocalScope();
    }

    bool IsStatusOK()
    {
        return ANI_OK == status_;
    }

    ani_status GetStatus()
    {
        return status_;
    }

private:
    ani_env *env_ = nullptr;
    ani_status status_ = ANI_ERROR;
};

} // namespace AniUtil
} // namespace OHOS

#endif
