#include <cstddef>
#include <iostream>
#include <ostream>
#include <string>
#include <unistd.h>

#include "gen/proto/api/robot/v1/robot.pb.h"
#include "gen/proto/api/robot/v1/robot.grpc.pb.h"
#include <grpcpp/grpcpp.h>

using grpc::Channel;
using grpc::ClientContext;
using grpc::Status;
using proto::api::robot::v1::ResourceNamesRequest;
using proto::api::robot::v1::RobotService;
using proto::api::robot::v1::ResourceNamesResponse;


extern "C" void *init_rust_runtime();
extern "C" int free_rust_runtime(void *ptr);
extern "C" void free_string(char* s);
extern "C" char *dial(const char *uri, const char *payload,
                             bool allow_insecure, void *ptr);

class RobotServiceClient {
public:
  RobotServiceClient(std::shared_ptr<Channel> channel)
      : stub_(RobotService::NewStub(channel)) {}

  void Resources() {
    ResourceNamesRequest req;

    ResourceNamesResponse resp;
    ClientContext context;

    Status status =
        stub_->ResourceNames(&context, req, &resp);
	if (!status.ok()) {
		std::cout << "Rpc failed "<< status.error_code() << status.error_message() << std::endl;
		return;
    }
	for(auto i = 0; i < resp.resources_size(); i++){
		std::cout << "Resource " << i << " " << resp.resources(i).type() << std::endl;
			}
  }

private:
  std::unique_ptr<RobotService::Stub> stub_;
};

int main(int argc, char *argv[]) {

  void *ptr = init_rust_runtime();
  char *path = dial("https://test-main.33vvxnbbw9.local.viam.cloud:8080",
					"9x375brdv1f7u2v6vi42a21cbzo0xuauov025wox5mg9grd5",
					false, ptr);
  if(path == NULL){
	  free_rust_runtime(ptr);
	  return 1;
  }
  std::cout << "Proxy at "
			<< path << std::endl;

  std::string address("unix://");
  address += path;
  RobotServiceClient client(
      grpc::CreateChannel(address, grpc::InsecureChannelCredentials()));

  client.Resources();
  
  free_rust_runtime(ptr);
  free_string(path);
  return 0;
}
