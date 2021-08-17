#/bin/sh
# Use this script to test it locally so you don't need to remove the folders every time
set -e

import_and_compile(){
  FILE=$1
  EXAM_NAME=$2
  rumbas import $FILE".exam"
  rumbas compile "exams/$EXAM_NAME.yaml"
}

import_and_compile_q(){
  FILE=$1
  QUESTION_NAME=$2
  rumbas import -q $FILE".exam"
  rumbas compile "questions/$QUESTION_NAME.yaml"
}

import_and_compile exam-110396-getting-started "Getting Started"
import_and_compile exam-115828-diagnosys "DIAGNOSYS"
import_and_compile exam-14065-geogebra-extension-demo "GeoGebra extension demo"
import_and_compile exam-119218-jesse-s-copy-of-simplex-method "Simplex method"
import_and_compile exam-11981-numbas-website-demo "Numbas website demo"
import_and_compile_q question-77526-written-number-extension "Written number extension"
import_and_compile_q question-91138-download-text-file-extension "Download text file extension"
import_and_compile_q question-79222-find-a-spanning-tree-in-an-undirected-graph "Find a spanning tree in an undirected graph"
import_and_compile_q question-7191-polynomials-extension "Polynomials extension"
import_and_compile_q question-102802-generate-a-system-of-linear-equations-to-solve "Generate a system of linear equations to solve" 
import_and_compile_q question-41128-find-a-basis-for-the-row-space-of-a-matrix "Find a basis for the row space of a matrix"
