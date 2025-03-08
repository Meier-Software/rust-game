#!/bin/bash
# Script to run the map editor

# Change to the script directory
cd "$(dirname "$0")"

# Activate the virtual environment
source venv/bin/activate

# Run the map editor with any arguments passed to this script
python map_editor.py "$@" 