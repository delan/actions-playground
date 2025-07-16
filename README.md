github actions playground
=========================

# exfiltrate secret with background process

- <https://github.com/delan/actions-playground/actions/workflows/exfiltrate_background.yml>

```
52.234.43.178 - - [16/Jul/2025:03:19:09 +0000] "GET /secret/pid_not_found HTTP/1.1" 404 435 "-" "-"
52.234.43.178 - - [16/Jul/2025:03:19:11 +0000] "GET /secret/pid_not_found HTTP/1.1" 404 435 "-" "-"
52.234.43.178 - - [16/Jul/2025:03:19:13 +0000] "GET /secret/pid_not_found HTTP/1.1" 404 435 "-" "-"
52.234.43.178 - - [16/Jul/2025:03:19:14 +0000] "GET /secret/found?value=oops HTTP/1.1" 404 435 "-" "-"
```
