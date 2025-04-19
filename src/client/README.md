
# Test false cases

```
// Double quote in the middle of key, the quotes should be part of the key
SET users["0"] tom
// Expect "tom"
GET users["0"]
```

```
// Key allow leading and trailing spaces
SET " k1 " " hello world "
// Expect " hello world "
GET " k1 "
```

```
// Key with escaped double quote
SET " k1\" " hello
// Expect "hello"
GET " k1\" "
```