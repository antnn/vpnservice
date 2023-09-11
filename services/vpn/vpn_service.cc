// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpn_service.h"

namespace vpn {

VpnService::VpnService(mojo::PendingReceiver<mojom::VpnService> receiver)
    : receiver_(this, std::move(receiver)) {}

VpnService::~VpnService() = default;

void VpnService::Divide(int32_t dividend,
                        int32_t divisor,
                        DivideCallback callback) {
  // Respond with the quotient!
  std::move(callback).Run(dividend / divisor);
}

}  // namespace vpn
