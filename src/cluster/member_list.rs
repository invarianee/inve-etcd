use crate::proto::etcdserverpb;
use crate::{Member, ResponseHeader};

#[derive(Debug, Clone)]
pub struct MemberListRequest {
    proto: etcdserverpb::MemberListRequest,
}

impl MemberListRequest {
    pub fn new() -> Self {
        Self {
            proto: etcdserverpb::MemberListRequest { linearizable: true },
        }
    }
}

impl From<MemberListRequest> for etcdserverpb::MemberListRequest {
    fn from(req: MemberListRequest) -> Self {
        req.proto
    }
}

#[derive(Debug, Clone)]
pub struct MemberListResponse {
    pub header: ResponseHeader,
    pub members: Vec<Member>,
}

impl From<etcdserverpb::MemberListResponse> for MemberListResponse {
    fn from(proto: etcdserverpb::MemberListResponse) -> Self {
        Self {
            header: From::from(proto.header.expect("must fetch header")),
            members: proto.members.into_iter().map(From::from).collect(),
        }
    }
}
