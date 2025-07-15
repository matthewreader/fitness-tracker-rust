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
    status)
        echo "Database status:"
        docker-compose ps
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|psql|status}"
        echo ""
        echo "Commands:"
        echo "  start   - Start the development database"
        echo "  stop    - Stop the development database"
        echo "  restart - Restart the development database"
        echo "  logs    - Show database logs"
        echo "  psql    - Connect to database with psql"
        echo "  status  - Show database status"
        exit 1
esac