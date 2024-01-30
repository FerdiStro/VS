#!/bin/bash

# Find the PID of the server process in the other container
SERVER_PID=$(docker-compose exec server pgrep -f "python -m http.server 8000")

if [ -n "$SERVER_PID" ]; then
    echo "Stopping server in the 'server' container with PID: $SERVER_PID"
    docker-compose exec server kill -SIGINT "$SERVER_PID"
else
    echo "Server process not found in the 'server' container."
fi

# Additional cleanup if needed
# ...

# Exit with success status
exit 0
