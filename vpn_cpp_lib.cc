’5// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "vpn_cpp_lib.h"

#include <string>
#include "base/at_exit.h"
#include "base/command_line.h"
#include "base/functional/bind.h"
#include "base/logging.h"
#include "base/strings/string_number_conversions.h"
#include "base/test/task_environment.h"
#include "base/test/test_future.h"
#include "base/test/test_timeouts.h"
#include "mojo/core/embedder/embedder.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/network/public/mojom/network_service.mojom.h"
#include "vpnservice/services/vpn/vpn_service.h"

mojo::Remote<network::mojom::NetworkService>* g_network_service_remote =
    nullptr;

network::mojom::NetworkService* GetNetworkService();

int vpn_main_cpp(int argc, char* argv[]) {
  base::AtExitManager exit_manager;
  base::CommandLine::Init(argc, argv);
  TestTimeouts::Initialize();
  base::test::TaskEnvironment task_environment{
      base::test::TaskEnvironment::TimeSource::SYSTEM_TIME};
  mojo::core::Init();

  if (argc <= 2) {
    LOG(INFO) << argv[0] << ": missing operand";
    return -1;
  }

  int dividend = 0;
  if (!base::StringToInt(argv[1], &dividend)) {
    LOG(INFO) << argv[0] << ": invalid dividend '" << argv[1] << "'";
    return -1;
  }

  int divisor = 0;
  if (!base::StringToInt(argv[2], &divisor) || divisor == 0) {
    LOG(INFO) << argv[0] << ": invalid divisor '" << argv[2] << "'";
    return -1;
  }

  // Create a mojo remote and pass a corresponding receiver to `VpnService`. In
  // a "real-world" situation the receiver and remote would typically be owned
  // by objects in different processes.

  // This process (remote)                                VpnService (receiver)
  // |  -> create pipe and pass receiver ->                       bind to pipe |
  // |  -> send Divide() call through pipe ->       received message from pipe |
  // | awaiting response from pipe...                             run Divide() |
  // | result received                 <- pass result across pipe to remote <- |
  mojo::Remote<vpn::mojom::VpnService> Vpn_service;
  vpn::VpnService Vpn_service_impl(Vpn_service.BindNewPipeAndPassReceiver());

  // `TestFuture` can be used to test code that return results asynchronously
  // through a callback. Prefer this to `RunLoop` in tests where possible!
  base::test::TestFuture<int32_t> future;
  Vpn_service->Divide(dividend, divisor, future.GetCallback());

  int32_t quotient = future.Get();
  LOG(INFO) << "Quotient: " << quotient;

  GetNetworkService();
  return 0;
}

void OnNetworkServiceCrash() {}

network::mojom::NetworkService* GetNetworkService() {
  if (!g_network_service_remote) {
    g_network_service_remote = new mojo::Remote<network::mojom::NetworkService>;
  }
  if (!g_network_service_remote->is_bound() ||
      !g_network_service_remote->is_connected()) {
    mojo::PendingReceiver<network::mojom::NetworkService> receiver =
        g_network_service_remote->BindNewPipeAndPassReceiver();
    g_network_service_remote->set_disconnect_handler(
        base::BindOnce(&OnNetworkServiceCrash));
    // CreateInProcessNetworkService(std::move(receiver));
  };
  return g_network_service_remote->get();
}


void msetnrtworlvontext( mojo::PendingReceiver<network::mojom::NetworkContext> context,
    network::mojom::NetworkContextParamsPtr params,
    SandboxGrantResult grant_access_result) {
      if (!network_context_ || !network_context_.is_connected()) {
      network_context_.reset();
      content::CreateNetworkContextInNetworkService(
          network_context_.BindNewPipeAndPassReceiver(),
          CreateNetworkContextParams());

  
  auto* network_service = GetNetworkService();
  network_service->CreateNetworkContext(std::move(context), std::move(params));
}








