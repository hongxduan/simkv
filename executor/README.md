## SET
set new value or update existing value

#### String sample
```
set k1 v1           // set key value paire k1 and v1
set k1 v1 -nx       // set key value paire k1 and v1 only if k1 not exist yet
set k1 v1 -ex       // set key value paire k1 and v1 on if k1 exist already
set k1 v1 -ttl num  // set key value paire k1 and v1 with [num] seconds time to live 
set k1 -ttl num     // update time to live of k1 
```

#### List sample
```
set hobbies[0] football     // set first value of list, equivalent push front
set hobbies[-1] swimming    // set last value of lsit, equivalent push end
set hobbies[3] jogging      // insert value at index 3, if the index exceed existing itme number, the INVALID_INDEX error
set hobbies[&1] hiking      // replace value at index 1, return old value if index is valid, else return INVALID_INDEX error 
```

## GET
#### String sample
```
get k1                      // get value of k1
get k1 -ttl                 // get ttl of k1 of string type
get k1 -del                 // get value of k1 and delete the k1
```
#### List sample
```
get hobbies[0]              // get first value of list
get hobbies[-1]             // get last value of list
get hobbies[3]              // the forth value of list
get hobbies[0] -del         // get first value and delete, equivalent pop from front
get hobbies[-1] -del        // get last value and delete, equivalent pop from end
get hobbies[] -ttl          // get ttl of list
get hobbies[1..3]           // get values from index 1 to 2, not include 3
get hobbies[..]             // get all values of list 
get hobbies[#]              // get length of list
get hobbies[&swimming]      // get index of swimming, return positive number if value exist, 
                            // or return -1 if value not exist
```

## DEL

## KEY