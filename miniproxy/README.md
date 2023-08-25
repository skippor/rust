# miniproy

## Flow

client1  <--->  proxy  <--->  client2

## Usage

```shell
miniproxy -f proxy.ini
```
```proxy.ini
[server]
port = 12345
token = 666666

[client]
remote = 1.2.3.4:12345
token = 123456

[proxy]
proto = UDP
lport = 11111
rport = 22222

```


## TODO

* crypto
* iprange-portrange