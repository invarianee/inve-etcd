use crate::proto::etcdserverpb;

#[derive(Debug, Clone)]
pub struct ResponseHeader {
    proto: crate::proto::etcdserverpb::ResponseHeader,
}

impl ResponseHeader {
    pub fn cluster_id(&self) -> u64 {
        self.proto.cluster_id
    }

    pub fn member_id(&self) -> u64 {
        self.proto.member_id
    }

    pub fn revision(&self) -> i64 {
        self.proto.revision
    }

    pub fn raft_term(&self) -> u64 {
        self.proto.raft_term
    }
}

impl From<etcdserverpb::ResponseHeader> for ResponseHeader {
    fn from(proto: etcdserverpb::ResponseHeader) -> Self {
        Self { proto }
    }
}
