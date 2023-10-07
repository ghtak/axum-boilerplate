from __future__ import print_function

import grpc

import voting_pb2
import voting_pb2_grpc


if __name__ == '__main__':
    with grpc.insecure_channel('localhost:18089') as channel:
        stub = voting_pb2_grpc.VotingStub(channel)
        response = stub.Vote(
            voting_pb2.VotingRequest(url="UrlValue", vote=voting_pb2.VotingRequest.Vote.UP)
        )
        print("Greeter client received: " + response.confirmation)
