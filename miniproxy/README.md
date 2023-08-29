# miniproy

## Flow

client1  <--->  proxy  <--->  client2

## Usage

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

## TODO

* crypto
* iprange-portrange