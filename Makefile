run_employee_db:
	@echo "Starting employee db service..."
	docker compose -f ./hex_employee_deployment/docker-compose.yaml stop hex-employee_pg_db
	docker compose -f ./hex_employee_deployment/docker-compose.yaml up --build -d hex-employee_pg_db
	@echo "employee auth db services started..."