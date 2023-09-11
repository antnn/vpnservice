// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "mojo/public/cpp/bindings/receiver.h"
#include "vpnservice/services/vpn/public/mojom/vpn_service.mojom.h"

namespace vpn {

class VpnService : public mojom::VpnService {
 public:
  explicit VpnService(mojo::PendingReceiver<mojom::VpnService> receiver);
  ~VpnService() override;
  VpnService(const VpnService&) = delete;
  VpnService& operator=(const VpnService&) = delete;

 private:
  // mojom::VpnService:
  void Divide(int32_t dividend,
              int32_t divisor,
              DivideCallback callback) override;

  mojo::Receiver<mojom::VpnService> receiver_;
};

}  // namespace vpn
