#!/bin/bash

# Ports to check
PORTS=(3000 3001)

for PORT in "${PORTS[@]}"; do
    # Find PID using lsof
    # -t: terse mode (only PIDs)
    # -i :PORT select internet files on PORT
    PIDS=$(lsof -ti :$PORT)
    
    if [ -n "$PIDS" ]; then
        echo "Found process(es) on port $PORT: $PIDS"
        # Split PIDS into array in case multiple exist
        for PID in $PIDS; do
            echo "Killing PID $PID..."
            kill -9 $PID
        done
        echo "Port $PORT cleared."
    else
        echo "No process found on port $PORT."
    fi
done
