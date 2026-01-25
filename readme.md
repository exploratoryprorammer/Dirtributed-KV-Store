## What is Raft?

Raft offers a generic way to distribute state machines across a cluster of computing systems to ensure each node in the cluster agrees upon the same state transitions 

Raft part: Leader, follower, and candidate
The leader will send a heartbeat. The follower requests a heartbeat from the leader every 150ms to 300ms. In the event the follower does not get a signal from the leader it will timeout and the follower will change its status to candidate and a leader election starts by the candidate.

Leader election:
In a leader election, the candidate node elects itself as a leader and sends a signal to all of the other nodes for a vote. If a candidate receives enough votes that the candidates with a term number higher than the current leader the candidate becomes the new leader. If the candidate looses then it accepts the legitimate winner of the election. In the event that there is a tie in the voting process, then a new term starts and new election begins. Raft uses ranodmized server tiemouts to ensure split votes wont happen as servers turn into candidates asyncronously.

Read more here: https://en.wikipedia.org/wiki/Raft_(algorithm)


