#!/bin/sh

jq_in_place(){
  COMMAND=$1
  FILE=$2
  jq $COMMAND $FILE > "$FILE.tmp"
  mv "$FILE.tmp" $FILE
}

clean(){
  FILE=$1
  NEW_FILE_NAME=$2
  tail -n+2 "$FILE" > $NEW_FILE_NAME
  jq_in_place 'del(.question_groups[].questions[].contributors)' $NEW_FILE_NAME
  jq_in_place 'del(.contributors)' $NEW_FILE_NAME
  jq_in_place 'del(.question_groups[].questions[].metadata)' $NEW_FILE_NAME
  jq_in_place 'del(.metadata)' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].extensions[]|=sub("'$NUMBAS_FOLDER'/extensions/";"")' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].extensions|=sort' $NEW_FILE_NAME
  jq_in_place '.extensions[]|=sub("'$NUMBAS_FOLDER'/extensions/";"")' $NEW_FILE_NAME
  jq_in_place '.extensions|=sort' $NEW_FILE_NAME
  jq_in_place '.resources[][1]|=sub(".*/question-resources/";"")' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].resources[][1]|=sub(".*/question-resources/";"")' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].tags|=map(select(.|startswith("skill:")))' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].tags|=sort' $NEW_FILE_NAME
  jq_in_place '.question_groups[].questions[].ungrouped_variables|=sort' $NEW_FILE_NAME
  #jq_in_place '.question_groups[].questions[].parts[]|.displayColumns?|select(.)|=tonumber' $NEW_FILE_NAME
  #jq_in_place '.question_groups[].questions[].parts[]|.maxMarks?|select(.)|=tonumber' $NEW_FILE_NAME
}

FILE_NUMBAS="tmp/"$3"-numbas.json"
FILE_RUMBAS="tmp/"$3"-rumbas.json"

clean "$1" $FILE_NUMBAS
clean "$2" $FILE_RUMBAS

jd $FILE_NUMBAS $FILE_RUMBAS > "tmp/"$3".patch"
