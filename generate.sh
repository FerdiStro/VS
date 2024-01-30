#!/bin/bash

# Check if docker-compose is installed
if command -v docker-compose > /dev/null; then

  # Add Temp-Files
  cp src/VC.html build-docker/input.html

  # Change to the build-docker directory
  cd build-docker/

  # Build using docker-compose
  if docker-compose up --build; then
    # Copy the generated PDF back to the parent directory
    cp output/output.pdf ../VC.pdf
    # Remove Temp-Files
    rm input.html
    sudo rm -rf output/

  else
    echo "Docker Compose build failed. Exiting without cleanup."
  fi

else
  echo "Install docker to build PDF"
fi

echo "Clean up completed."

