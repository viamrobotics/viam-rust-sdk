#include <cstddef>
#include <iostream>
#include <ostream>
#include <string>
#include <unistd.h>

#include "proto/rpc/examples/echo/v1/echo.grpc.pb.h"
#include "proto/rpc/examples/echo/v1/echo.pb.h"
#include <grpcpp/grpcpp.h>

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;

using proto::rpc::examples::echo::v1::EchoRequest;
using proto::rpc::examples::echo::v1::EchoResponse;
using proto::rpc::examples::echo::v1::EchoService;

extern "C" void *init_rust_runtime();
extern "C" int free_rust_runtime(void *ptr);
extern "C" void free_string(char* s);
extern "C" char *dial_direct(const char *uri, const char *payload,
                             bool allow_insecure, void *ptr);

class EchoServiceClient {
public:
  EchoServiceClient(std::shared_ptr<Channel> channel)
      : stub_(EchoService::NewStub(channel)) {}

  std::string Echo(const std::string &str) {
    EchoRequest req;
    req.set_message(str);

    EchoResponse resp;
    ClientContext context;

    Status status = stub_->Echo(&context, req, &resp);
    if (status.ok()) {
      return resp.message();
    } else {
      return "Rpc failed";
    }
  }

private:
  std::unique_ptr<EchoService::Stub> stub_;
};

int main(int argc, char *argv[]) {

  void *ptr = init_rust_runtime();
  char *path = dial_direct("http://127.0.0.1:55133", NULL, true, ptr);
  if(path == NULL){
	  free_rust_runtime(ptr);
	  return 1;
  }
  std::cout << "Proxy at "
			<< path << std::endl;

  std::string address("unix://");
  address += path;
  EchoServiceClient client(
      grpc::CreateChannel(address, grpc::InsecureChannelCredentials()));
  std::cout << "Echo  " << client.Echo("Hello") << std::endl;

  free_rust_runtime(ptr);
  free_string(path);
  return 0;
}
