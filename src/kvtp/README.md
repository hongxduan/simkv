
# Key Value Transport Protocal

Key Value Transport Protocal is the message format standard use for Key Value transport between Client and Server

KVTP contains two parts, Header and Body, Separated by a `empty` line

Informations in KVTP are divided into `lines` by Line Feed `\n`


## Request
```
KVTP/1              ...... Protocol (Must be the first line)
CMD: GET|SET        ...... Command
KEY: k1|users[1]    ...... Key
ARGS: EX|NX         ...... Arguments (Optional)
TTL: 0              ...... Time To Live (Optional)
                    ...... Empty line(Header Body separator)
Body                ...... Body
```

### Body
Body is squence of bytes, body may container multiple items

#### item format
- Item header: fixed 4 bytes, indicate the item length in terms of bytes
- Item body: N bytes of item content
```
+-----------------------------+
| 4 bytes |     item body     |
+-----------------------------+
```

## Response
```
KVTP/1 OK|ERR       ...... Protocol and Status(Must be the first line)
DTYPE: I|L|D...     ...... Data type
                    ...... Empty line(Header Body separator)
Body                ...... Body
```

### Data Types
- I: 32 bit Integer
- L: 64 bit Integer
- D: Double
- S: String
- LI: List of Integer
- LL: LIST of Long
- LD: LIST OF Double
- LS: List of String
- M: Map

### Parse Integer(I) body
Convert the whole body from big-endian bytes to Integer 

### Parse String(S) body
Convert the whole body from bytes to String

### Parse List of Integer(LI) body
- Each List item consists of fixed 4 bytes
- Read every 4 bytes and convert from big-endian bytes to Integer
```
+-------------------------+
| 4 bytes | 4 bytes | ... |
+-------------------------+
```

### Parse List of String(LS) body
- Each List item consists of fixed 4 bytes header, and N bytes value, N depends on the first 4 bytes 
- Read every 4 bytes header to get value length, and then the corespondent value
```
+-----------------+
| 4 bytes | value |
+-----------------+
```

### Map
- Each Map entry consists of fixed 4 bytes of field length, followed by field
- And then 4 bytes value length, followed by value
```
+-----------------------------------+
| 4 bytes | field | 4 bytes | value |
+-----------------------------------+
```
