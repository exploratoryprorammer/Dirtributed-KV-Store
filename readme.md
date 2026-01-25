## What is Raft?

Raft offers a generic way to distribute state machines across a cluster of computing systems to ensure each node in the cluster agrees upon the same state transitions 

Raft part: Leader, follower, and candidate
The leader will send a heartbeat. The follower requests a heartbeat from the leader every 150ms to 300ms. In the event the follower does not get a signal from the leader it will timeout and the follower will change its status to candidate 

