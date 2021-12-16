#/bin/sh
set -e

rumbas init

# Set resources
cp -r numbas_resources resources/question-resources

cp question_preview.yaml exam_templates/

./run.sh
