tonic::include_proto!("voting");

use tonic::{Request, Response, Status};

use self::voting_server::Voting;



#[derive(Debug, Default)]
pub(crate) struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        let r = request.into_inner();
        match r.vote {
            0 => Ok(Response::new(VotingResponse {
                confirmation: { format!("Happy to confirm that you upvoted for {}", r.url) },
            })),
            1 => Ok(Response::new(VotingResponse {
                confirmation: { format!("Confirmation that you downvoted for {}", r.url) },
            })),
            _ => Err(Status::new(
                tonic::Code::OutOfRange,
                "Invalid vote provided",
            )),
        }
    }
}
