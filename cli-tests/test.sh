#/bin/sh
set -e

rumbas init

# Set resources
mkdir resources/question-resources
cp material-dw4n67kf_vCI9659.ggb resources/question-resources

import_and_compile(){
  FILE=$1
  EXAM_NAME=$2
  rumbas import $FILE".exam"
  rumbas compile "exams/$EXAM_NAME.yaml"
}


import_and_compile exam-110396-getting-started "Getting Started"
import_and_compile exam-115828-diagnosys "DIAGNOSYS"
import_and_compile exam-14065-geogebra-extension-demo "GeoGebra extension demo"
