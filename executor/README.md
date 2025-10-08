## SET
set new value or update existing value

#### String Sample
```
set k1 v1           // set key value paire k1 and v1
set k1 v1 -nx       // set key value paire k1 and v1 only if k1 not exist yet
set k1 v1 -ex       // set key value paire k1 and v1 on if k1 exist already
set k1 v1 -ttl num  // set key value paire k1 and v1 with [num] seconds time to live 
set k1 -ttl num     // update time to live of k1 
```

#### List Sample
```
set hobbies[0] football     // left(front) push 
set hobbies[-1] swimming    // right(back) push 
set hobbies[3] jogging      // insert value at index 3, if the index exceed existing itme number, the INVALID_INDEX error
set hobbies[&1] hiking      // replace value at index 1, return old value if index is valid, else return INVALID_INDEX error 
```

## GET

## DEL

## KEY