# Start redpanda cluster (redpanda-2 depends on redpanda-1, which depends on redpanda-0) & Triton
docker compose up -d redpanda-2 loki grafana prometheus scylla-0

# If you're using the HA scylla configuration in docker-compose, use this instead:
# docker compose up -d redpanda-2 loki grafana prometheus scylla-2

# Wait a lil bit for redpanda to come up before executing config commands
echo "Waiting 45 sec for Redpanda and ScyllaDB to start!"
sleep 45s

# Create topics & shrink segment sizes so they trigger automatic archiving

## Simple sensor - 250mb segments
docker exec redpanda-0 rpk topic create -r 3 -p 5 raw.test.simple
docker exec redpanda-0 rpk topic alter-config raw.test.simple --set segment.bytes=250000000

## Simple model - 250mb segments
docker exec redpanda-0 rpk topic create -r 3 -p 5 derived.model-simple.add-subtract
docker exec redpanda-0 rpk topic alter-config derived.model-simple.add-subtract --set segment.bytes=250000000

# disable redpanda anonymous telemetry
docker exec redpanda-0 rpk redpanda config set rpk.enable_usage_stats false
echo "Disabled anonymous telemetry reporting"

# Check status of the cluster
docker exec redpanda-0 rpk cluster config status

# let you read the topic creation and cluster status before continuing

# Bring up the rest of the stack
docker compose up -d