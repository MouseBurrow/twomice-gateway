# Gateway Service — Route Map

The gateway is a reverse proxy. It does not serve API routes itself — it forwards requests to upstream services based on path prefix matching.

**External URL pattern:** `https://host/api/<path>` → Caddy/nginx strips `/api` → gateway receives `/path`

---

## Routing table

| Path prefix | Upstream | Forwarded to |
|---|---|---|
| `/login` | `auth:8080` | `POST /login` |
| `/signup` | `auth:8080` | `POST /signup` |
| `/logout` | `auth:8080` | `POST /logout` |
| `/account` | `auth:8080` | `GET /account` |
| `/b` | `post:8080` | `/b*` (boards, posts, comments, replies, votes) |
| `/feed` | `post:8080` | `GET /feed` |
| `/users/me/nibs` | `post:8080` | `GET /users/me/nibs` |
| `/moderation` | `moderation:8080` | `/moderation*` |
| `/social` | `social:8080` | `/social*` |
| `/users` | `social-feed:8080` | `/users*` (following, stats) |
| anything else | — | **404 Not Found** |

Order matters: `/users/me/nibs` is checked before `/users` (general), so user nibs routes to post service, but other `/users` paths route to social-feed.

---

## Auth flow

```
Browser request (with cookie)
  → Gateway extracts `session_token` from Cookie header
  → Gateway calls POST /validate on auth service with X-Session-Token
  → Auth responds with { user_id: "12345" } or null
  → Gateway caches result (1 hour TTL)
  → Gateway forwards request to upstream with X-User-Id header
  → Upstream service uses UserId or OptionalUserId extractor
```

**Unauthenticated requests:** If no session cookie, requests proceed without `X-User-Id`.

**Logout:** Request forwarded first; on success the cached token is invalidated.

---

## Error responses

| Condition | Status | Body |
|---|---|---|
| No matching route | 404 | `"not found"` |
| Auth service unreachable | 502 | `"auth error"` |
| Upstream request fails | 502 | `"upstream error: {msg}"` |
| Upstream body read fails | 502 | `"body error: {msg}"` |
| Catch-all failure | 502 | `"bad gateway"` |

---

## Service URLs (configurable via env)

| Variable | Default |
|---|---|
| `AUTH_SERVICE_URL` | `http://auth:8080` |
| `POST_SERVICE_URL` | `http://post:8080` |
| `MODERATION_SERVICE_URL` | `http://moderation:8080` |
| `SOCIAL_SERVICE_URL` | `http://social:8080` |
| `FEED_SERVICE_URL` | `http://social-feed:8080` |
