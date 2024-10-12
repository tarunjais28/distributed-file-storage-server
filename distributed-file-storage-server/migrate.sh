#!/bin/sh

# Run Diesel migrations (point to the correct migrations directory)
cd /usr/src/app
/usr/local/bin/diesel migration run

# Start the application
exec /usr/local/bin/distributed-file-storage-server
