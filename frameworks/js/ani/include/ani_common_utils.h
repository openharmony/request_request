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

#ifndef ANI_COMMON_UTILS_H
#define ANI_COMMON_UTILS_H

#include <exception>
#include <memory>
#include <type_traits>
#include <utility>
#include <variant>

template<typename T>
class SharedPtrHolder {
public:
    SharedPtrHolder(std::shared_ptr<T> &sptr) : sptr_(sptr)
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


template <typename F>
class FinalAction {
public:
    explicit FinalAction(F func) : func_(std::move(func))
    {
    }

    ~FinalAction() noexcept(noexcept(func_()))
    {
        if (!dismissed_) {
            func_();
        }
    }

    FinalAction(const FinalAction&) = delete;
    FinalAction& operator=(const FinalAction&) = delete;

    FinalAction(FinalAction&& other) noexcept
        : func_(std::move(other.func_)),
          dismissed_(other.dismissed_)
    {
        other.dismissed_ = true;
    }

    void dismiss() noexcept
    {
        dismissed_ = true;
    }

private:
    F func_;
    bool dismissed_ = false;
};

template <typename F>
inline FinalAction<F> finally(F&& func)
{
    return FinalAction<F>(std::forward<F>(func));
}

#endif
