#!/bin/bash
# set -e

echo "--------------------------------------"
echo "   RelayAI Infrastructure Wizard      "
echo "--------------------------------------"

RABBIT_HOST=${AMQP_HOST:-127.0.0.1}
OLLAMA_URL=${OLLAMA_HOST:-http://127.0.0.1:11434}

echo "Checking RabbitMQ at $RABBIT_HOST:5672..."
if ! timeout 1s bash -c "true > /dev/tcp/$RABBIT_HOST/5672" 2>/dev/null; then
  echo "RabbitMQ not detected."
  read -p "Would you like to pull and run RabbitMQ now? (y/n): " pull_rabbit
  if [[ "$pull_rabbit" == "y" || "$pull_rabbit" == "Y" ]]; then
    docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management
    echo " RabbitMQ started. Waiting 5s..."
    sleep 5
  fi
else
  echo "RabbitMQ is already UP."
fi

echo " Checking Ollama at $OLLAMA_URL..."
if ! curl -s --connect-timeout 2 "$OLLAMA_URL/api/tags" > /dev/null; then
  echo " Ollama not detected."
  read -p "Would you like to pull and run Ollama now? (y/n): " pull_ollama
  if [[ "$pull_ollama" == "y" || "$pull_ollama" == "Y" ]]; then
    docker run -d --name ollama -p 11434:11434 ollama/ollama
    echo "Ollama started. Pulling model..."
    docker exec -d ollama ollama pull llama3.2:1b
  fi
else
  echo "Ollama found"
fi

echo "Launching..."

if [ $# -eq 0 ]; then
    echo "Done! No binary specified to run."
else
    exec "$@"
fi