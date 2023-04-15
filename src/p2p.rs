use warp::{Filter, Reply, Rejection};
use serde::{Serialize, Deserialize};

type Result<T> = std::result::Result<T, Rejection>;
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ResponseBody1 {
    term: u32,
    voteGranted: bool,
}
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ResponseBody2 {
    term: u32,
    success: bool,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct RequestVote {
    term: u32,
    candidateId: u32,
    lastLogIndex: i32,
    lastLogTerm: u32,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct AppendEntries {
    term: u32,
    leaderId: u32,
    prevLogIndex: i32,
    prevLogTerm: u32,
    entries: Vec<Entries>,
    leaderCommit: u32
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Entries{
        term: u32,
        command: String
}

#[tokio::main]
pub async fn p2p() {

    let route_req_vote = warp::path("requestVote")
        .and(warp::post())
        .and(warp::body::json())
/*        .map(|| {
            let my_struct = MyStruct{ term: 3, vote_granted: true};
            warp::reply::json(&my_struct)
        });*/
        .and_then(handler_request_vote);

    let route_append_entries = warp::path("appendEntries")
        .and(warp::post())
        .and(warp::body::json())
/*        .map(|| {
            let my_struct = MyStruct2{ term: 3, success: true};
            warp::reply::json(&my_struct)
        });*/
        .and_then(handler_append_entries);
    let route_execute_command = warp::path("executeCommand")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024*32))
        .and(warp::body::bytes())
        .map(|body: bytes::Bytes| String::from_utf8_lossy(&body).to_string());

    let routes = warp::post().and(
        route_req_vote
            .or(route_append_entries)
            .or(route_execute_command)
    );
    warp::serve(routes)
        .run(([127, 0, 0, 1 ], 3031))
        .await;
}


pub async fn handler_request_vote(body: RequestVote) -> Result<impl Reply> {
    let response_body = ResponseBody1 {
        term: body.term,
        voteGranted: true
    };
    //Ok(format!("Called with {:?}", body))
    Ok(warp::reply::json(&response_body))
}

pub async fn handler_append_entries(body: AppendEntries) -> Result<impl Reply> {
    let response_body = ResponseBody2 {
        term: body.term,
        success: true
    };
    Ok(warp::reply::json(&response_body))
    //Ok(format!("Called with {:?}", body))
}