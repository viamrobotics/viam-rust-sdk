LDFLAGS = -L/usr/local/lib `pkg-config --libs protobuf grpc++ re2 libcares`\
-ldl -L../../../../target/debug -lviam

CXX = g++
CXXFLAGS += `pkg-config --cflags protobuf grpc`
CXXFLAGS += -std=c++11 -I gen/ -I gen/proto/

all: buf ffi_echo

ffi_echo: gen/google/api/http.pb.o gen/google/api/annotations.pb.o gen/proto/rpc/examples/echo/v1/echo.pb.o gen/proto/rpc/examples/echo/v1/echo.grpc.pb.o ffi_echo.o
	$(CXX) $^ $(CXXFLAGS) $(LDFLAGS) -o $@

ffi_robot: gen/google/api/http.pb.o gen/google/api/annotations.pb.o gen/robot/v1/robot.pb.o gen/robot/v1/robot.grpc.pb.o gen/common/v1/common.pb.o gen/common/v1/common.grpc.pb.o ffi_robot.o
	$(CXX) $^ $(CXXFLAGS) $(LDFLAGS) -o $@

buf:
	@mkdir -p gen
	buf generate buf.build/viamrobotics/goutils --template buf.gen.yaml
	buf generate buf.build/googleapis/googleapis --template buf.gen.yaml --path google/rpc --path google/api 
	buf generate buf.build/viamrobotics/api --template buf.gen.yaml
clean:
	rm -f ffi_echo *.o
	rm -rf gen/*
