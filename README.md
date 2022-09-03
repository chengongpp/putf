# Putf

## Why

Serve file through TCP like netcat but with a nicer interface.

## Usage

Server side:

```bash
# Assuming server ip is 192.168.19.19 and we're gonna transfer frpc
./putf -l 0.0.0.0:11451 -f /tmp/frpc
```

Client side:

```bash
cat < /dev/tcp/192.168.19.19/11451 > frpc
```

Don't forget to `md5sum frpc` to check file integrity.
