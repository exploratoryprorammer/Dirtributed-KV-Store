## What is Raft?

Raft offers a generic way to distribute state machines across a cluster of computing systems to ensure each node in the cluster agrees upon the same state transitions 

Raft part: Leader, follower, and candidate
The leader will send a heartbeat. The follower requests a heartbeat from the leader every 150ms to 300ms. In the event the follower does not get a signal from the leader it will timeout and the follower will change its status to candidate and a leader election starts by the candidate.

Leader election:
In a leader election, the candidate node elects itself as a leader and sends a signal to all of the other nodes for a vote. If a candidate receives enough votes that the candidates with a term number higher than the current leader the candidate becomes the new leader. If the candidate looses then it accepts the legitimate winner of the election. In the event that there is a tie in the voting process, then a new term starts and new election begins. Raft uses ranodmized server tiemouts to ensure split votes wont happen as servers turn into candidates asyncronously.

Log Replication:
The leader accepts client requests for commands for the state machine in the cluster. After the log being appended to the leaders logs as a new entry, the request is forwarded to the followers as AppendEntries messages. The leader tries to send the AppendEntries messages indefinetly in case the followers are unavailble until the log is stored by all of its followers. After confimation that more than half of the leaders followers have replicated the entry, the leader will apply the entry to its local state machine and the request is considered commited. Once the followers learns this they will do the same. 
A leader can crash and therefore cause logs to be left inconsistent. In this case, the new leader will handle inconsistencies by forcing followers to duplicate its logs. The leader will find the last entry where they agree with each of its followers and then delete all of the entires after that entry in its followers logs and replace it with its own logs.

Read more here: https://en.wikipedia.org/wiki/Raft_(algorithm)


