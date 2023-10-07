# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import voting_pb2 as voting__pb2


class VotingStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Vote = channel.unary_unary(
                '/voting.Voting/Vote',
                request_serializer=voting__pb2.VotingRequest.SerializeToString,
                response_deserializer=voting__pb2.VotingResponse.FromString,
                )


class VotingServicer(object):
    """Missing associated documentation comment in .proto file."""

    def Vote(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_VotingServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Vote': grpc.unary_unary_rpc_method_handler(
                    servicer.Vote,
                    request_deserializer=voting__pb2.VotingRequest.FromString,
                    response_serializer=voting__pb2.VotingResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'voting.Voting', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Voting(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def Vote(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/voting.Voting/Vote',
            voting__pb2.VotingRequest.SerializeToString,
            voting__pb2.VotingResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
