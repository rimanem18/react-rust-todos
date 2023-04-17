up:
	docker compose up -d
build:
	docker compose build --no-cache --force-rm
init:
	docker compose exec app chown 1000:1000 /work/backend -R
	docker compose exec web chown 1000:1000 /home/node/app -R
stop:
	docker compose stop
down:
	docker compose down --remove-orphans
restart:
	@make down
	@make up
rebuild:
	@make down
	@make build
	@make up
destroy:
	docker compose down --rmi all --volumes --remove-orphans
destroy-volumes:
	docker compose down --volumes --remove-orphans
ps:
	docker compose ps
app:
	docker compose exec app bash
web:
	docker compose exec web ash
