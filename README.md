## Command

### docker image

```bash
$ docker compose build --no-cache
$ docker-compose up -d
```

### connect api

```bash
$ docker-compose exec api bash
```

### connect postgres

```bash
$ docker-compose exec db bash
$ psql -U local_user -d webapi_mvp
```

### migrate

```bash
$ diesel migration run
```

### postgres

テーブル一覧

```bash
$ \dt
```
