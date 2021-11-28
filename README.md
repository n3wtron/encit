# EncIT # 

Offline e2e encryption CLI

## Commands ##

### Create an identity ###

```bash
$ encit new identity --help                                                                               1 ↵
encit-new-identity 

USAGE:
    encit new identity --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --name <name> 
```

### Example

```bash
encit new identity --name myself
```

### Get identities

```bash
$ ./encit get identities
myself
```

### Get single identity

```bash
$ encit get identity --help
USAGE:
    encit get identity [FLAGS] <name> --format <format>

FLAGS:
    -h, --help           Prints help information
        --private-key    display private key
    -V, --version        Prints version information

OPTIONS:
    -f, --format <format>     [default: hex-pem]  [possible values: pem, hex-pem, base64-pem]

ARGS:
    <name>  
```

#### get identity in hex pem format

```bash
$ ./encit get identity myself
Identity: myself
Public Key: 2d2d2d2d2d424547494e205055424c4943204b45592d2d2d2d2d0a4d494942496a414e42676b71686b6947397730424151454641414f43415138414d49494243674b43415145417a4738383332695869574f516a4b3638706174650a31764466506f684f784a376c36647a6e4d3247734a42774751776d7043354638447230562b6731376f4342746e6e30375530664c7949686a335774562f707a6f0a6c6c48586d576c4f42567372446e6a7070587856797a56434f5945326a33326a35504556327a4f69594379684b5a6156687a4c735963653453786172766b61340a58564b2b4a64334b6943553355347a544d4666474d63704539304141614278512b38426e6265472f65486777674344517830774161423654576c6c33794551710a766b312f6b4830515a744339304f7070734542593650793646426b50354651492b52537a5a4a2b6f44565a44694b536e766b6431617248387a623456575a37370a39483656792b4c5059396533493874315047512b6a5372713051797642726f756342564255626a67755a6931303643497a584462484f35367461753342512f4a0a48514944415141420a2d2d2d2d2d454e44205055424c4943204b45592d2d2d2d2d0a
```

#### get identity in pem format

```bash
$ ./encit get identity --format pem myself 
Identity: myself
Public Key: -----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAzG8832iXiWOQjK68pate
1vDfPohOxJ7l6dznM2GsJBwGQwmpC5F8Dr0V+g17oCBtnn07U0fLyIhj3WtV/pzo
llHXmWlOBVsrDnjppXxVyzVCOYE2j32j5PEV2zOiYCyhKZaVhzLsYce4Sxarvka4
XVK+Jd3KiCU3U4zTMFfGMcpE90AAaBxQ+8BnbeG/eHgwgCDQx0wAaB6TWll3yEQq
vk1/kH0QZtC90OppsEBY6Py6FBkP5FQI+RSzZJ+oDVZDiKSnvkd1arH8zb4VWZ77
9H6Vy+LPY9e3I8t1PGQ+jSrq0QyvBroucBVBUbjguZi106CIzXDbHO56tau3BQ/J
HQIDAQAB
-----END PUBLIC KEY-----
```

### Add a friend

```bash
$ encit add friend --help
USAGE:
    encit add friend [FLAGS] --format <format> --name <name> [key-file]

FLAGS:
    -h, --help       Prints help information
        --stdin      read the public key from stdin
    -V, --version    Prints version information

OPTIONS:
    -f, --format <format>     [possible values: pem, hex-pem, base64-pem]
    -n, --name <name>        

ARGS:
    <key-file>    key file
```

#### Example Add friend from public key file

```bash
$ encit add friend --format pem --name my-best-friend best-friend.pub.pem
```

#### Example Add friend from stdin

```bash
$ cat my-best-friend.pub.pem | encit add friend --format pem --name my-best-friend --stdin
```

### Get friends

```bash
$ encit get friends
my-best-friend
```


### Encrypt a message
The encrypted message contains the friend information and also the identity public key,
the receiver has to contain the sender in his friend list to be able to decrypt and verify the message.

```bash
$ encit encrypt --help
USAGE:
    encit encrypt [OPTIONS] --friend <friend> --identity <identity> [file]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --friend <friend>        Friend name (has to be present in the encit configuration file)
    -i, --identity <identity>    Identity name (has to be present in the encit configuration file)
    -s, --subject <subject>      Message subject

ARGS:
    <file>    file to encrypt
```

#### Example 
```bash
$ encit encrypt -f my-best-friend -i myself my-secrets.txt > my-secrets.txt.enc
```

### Decrypt
```bash
$ encit decrypt --help
USAGE:
    encit decrypt [FLAGS] [OPTIONS] [file]

FLAGS:
    -h, --help       Prints help information
        --json       
    -V, --version    Prints version information

OPTIONS:
    -i, --identity <identity>    Identity name (has to be present in the encit configuration file)

ARGS:
    <file>    file to encrypt
```

#### Example
```bash
$ encit decrypt my-secrets.txt.enc > my-fiend-secrets.txt
```
