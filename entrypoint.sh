#!/bin/sh
# Copy themes to numbas folder if there are themes
if [ -n "$(ls -A /rumbas/themes)" ]; then
  cp -r /rumbas/themes/* $NUMBAS_FOLDER/themes/
fi
# Run rumbas
cd /rumbas && rumbas "$@"
# Copy output
cp -r $NUMBAS_FOLDER/output/output /rumbas/output

