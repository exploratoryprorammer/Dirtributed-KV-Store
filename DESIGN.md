# Node Designs 

## Persistant State:
current_term 
voted_for
log[]

## Volatile State: 
commit_index 
last_applied

## Leader only 
next_index[]
match_index[]

## Distributed Systems file structure

main.rs: bootstraps cluster + event loop
node.rs: Raft state machine logic 
rpc: Raft RPC definitions/data structures
transport.rs: in-memory "network"
storage.rs: persistent state (stub for now)