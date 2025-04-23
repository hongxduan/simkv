
# Raft

## Role

### Leader
At any time point, there should be only one Raft leader

The leader elected either by cluster init, or by win the Vote Request

Leader keep sending heartbeat to all followers in random `[200, 400]` ms

### Follower


## Requests

### Leader heartbeat

Leader keep sending heartbeat to all followers

### VoteRequest

Follower start a background task to check if have received heartbeat of leader in the past `500` ms, if *no*, then start to send vote request

