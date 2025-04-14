


# List

List `Key` consist of two parts, the Key and ending square bracket
For example, a user list key can be `users[]`

## SET

### Push front(left)
```
// 0 means the first element in list
set users[0] tom
```

### Push back(right)
```
// -1 means the last element in list
set users[-1] tom
```

### Insert(middle)
```
// Insert `jerry` at index 5
// if the index(5) great than list length, return index out of bound error, i.e. IDX_UOB
// if the index(5) equals to length of list, then below command equals to Push back `set users[-1] jerry`
set users[5] jerry
```

### Replace 
```
// the user list [tom, jerry, spike]
// below command replace jerry with butch
// notice `$` before `1`
set users[$1] butch
```

### Push multiple value in single command
```
// push `tom` and `jerry` from front
// the list will be [jerry tom]
set users[0] tom jerry
```


## GET

### Pop front(left)
```
// the user list [tom, jerry, doggy]
// below command will return `tom`
// and the list after pop is [jerry, doggy]
get users[0]
```

### Pop back(right)
```
// the user list [tom, jerry, doggy]
// below command will return `doggy`
// and the list after pop is [tom, jerry]
get users[-1]
```

### Pop at(middle)
```
// the users list [tom, jerry, doggy]
// below command will return `jerry`
// and the list after pop is [tom, doggy]
get users[1]
```

### Get length of list
```
get users[#]
```

### Get index of element
```
// the users list [tom, jerry, doggy]
// below command will return 1
// notice the queustion mark `?` before `jerry`, it means where is `jerry`
// if the value not in the list, then return Err(NOT_FOUND)
get users[?jerry]
```