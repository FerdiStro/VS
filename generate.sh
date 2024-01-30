#!/bin/bash

if command -v docker-compose > /dev/null; then


  mkdir -p build-docker/build/assets/
  mkdir -p build-docker/output


  cp src/VC.html build-docker/build/input.html
  cp -r src/assets/ build-docker/build/


  cd build-docker/

  docker-compose up --build -d

  while true; do
    status=$(docker-compose ps -q html-to-pdf)
    if [ -z "$status" ]; then
      echo "html-to-pdf-1 exited with code 0"
      break
    fi
    sleep 1
  done

  if docker-compose logs | grep -q "Docker Compose build failed"; then
    echo "Docker Compose build failed. Exiting without cleanup."
  else
    cp output/output.pdf ../VC.pdf
    rm -rf build/
    rm -rf output/
    echo "Clean up completed."
  fi

else
  echo "Install docker-compose to build PDF"
fi
