#/bin/sh
# Use this script to test it locally so you don't need to remove the folders every time
set -e

import_and_compile(){
  FILE=$1
  EXAM_NAME=$2
  rumbas import $FILE".exam"
  rumbas compile "exams/$EXAM_NAME.yaml"
}


import_and_compile exam-110396-getting-started "Getting Started"
import_and_compile exam-115828-diagnosys "DIAGNOSYS"
import_and_compile exam-14065-geogebra-extension-demo "GeoGebra extension demo"
