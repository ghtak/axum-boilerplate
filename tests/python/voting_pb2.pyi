from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class VotingRequest(_message.Message):
    __slots__ = ["url", "vote"]
    class Vote(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
        __slots__ = []
        UP: _ClassVar[VotingRequest.Vote]
        DOWN: _ClassVar[VotingRequest.Vote]
    UP: VotingRequest.Vote
    DOWN: VotingRequest.Vote
    URL_FIELD_NUMBER: _ClassVar[int]
    VOTE_FIELD_NUMBER: _ClassVar[int]
    url: str
    vote: VotingRequest.Vote
    def __init__(self, url: _Optional[str] = ..., vote: _Optional[_Union[VotingRequest.Vote, str]] = ...) -> None: ...

class VotingResponse(_message.Message):
    __slots__ = ["confirmation"]
    CONFIRMATION_FIELD_NUMBER: _ClassVar[int]
    confirmation: str
    def __init__(self, confirmation: _Optional[str] = ...) -> None: ...
