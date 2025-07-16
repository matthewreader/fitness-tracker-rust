#!/bin/bash

# Development database management script for fitness-tracker-rust

case "$1" in
    start)
        echo "Starting PostgreSQL development database..."
        docker-compose up -d
        echo "Waiting for database to be ready..."
        sleep 5
        echo "Database is ready at: postgresql://postgres:password@localhost:5432/fitness_tracker"
        ;;
    stop)
        echo "Stopping PostgreSQL development database..."
        docker-compose down
        ;;
    restart)
        echo "Restarting PostgreSQL development database..."
        docker-compose down
        docker-compose up -d
        echo "Waiting for database to be ready..."
        sleep 5
        echo "Database is ready at: postgresql://postgres:password@localhost:5432/fitness_tracker"
        ;;
    logs)
        echo "Showing database logs..."
        docker-compose logs -f postgres
        ;;
    psql)
        echo "Connecting to database..."
        docker exec -it fitness-tracker-postgres psql -U postgres -d fitness_tracker
        ;;
    migrate)
        echo "Running migrations..."
        if [ ! -f ".env" ]; then
            echo "Error: .env file not found. Please create it with DATABASE_URL."
            exit 1
        fi
        cargo run -- migrate || echo "Run 'cargo install sqlx-cli' if you want to use sqlx migrate directly"
        ;;
    reset)
        echo "Resetting database (WARNING: This will delete all data!)"
        read -p "Are you sure? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            docker-compose down
            docker volume rm fitness-tracker-rust_postgres_data 2>/dev/null || true
            docker-compose up -d
            echo "Waiting for database to be ready..."
            sleep 5
            echo "Database reset complete"
        else
            echo "Database reset cancelled"
        fi
        ;;
    status)
        echo "Database status:"
        docker-compose ps
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|psql|migrate|reset|status}"
        echo ""
        echo "Commands:"
        echo "  start   - Start the development database"
        echo "  stop    - Stop the development database"
        echo "  restart - Restart the development database"
        echo "  logs    - Show database logs"
        echo "  psql    - Connect to database with psql"
        echo "  migrate - Run database migrations"
        echo "  reset   - Reset database (deletes all data)"
        echo "  status  - Show database status"
        exit 1
esac