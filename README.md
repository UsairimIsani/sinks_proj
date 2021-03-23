# MSINKS

```yaml
name: Hello

throtle: 100/min


sink:
   type: http
   host: 192.168.0.1
   port: 3030

endpoint: /endpoint_name

stores:
 -
    type: postgresql
    host: localhost
    port: 2181
    database: db_name

 -
    type: elastic
    host: localhost
    port: 2181
    index: index_name

License:
  registered_to: ABC Company
  subscription_expires: mm/dd/yyyy
  license_hash: hmac(ABC Company & mm/dd/yyyy)
```