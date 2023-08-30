# miniproy

## Flow

client1  <--->  proxy  <--->  client2

## Usage

```bash
miniproxy server -l 1.2.3.4:12345
miniproxy client -c 1.2.3.4:12345 -m 80:8080
```

## TODO

```module
config:
  hash<token> = config    -> tunnel create
route:
  btree<tupple> -> name   -> tunnel query
tunnel:
  hash<name> = tunnel     -> fd + crypto
crypto:
  enc/dec/hash/hmac
packet:
  recv/send               redirect
```