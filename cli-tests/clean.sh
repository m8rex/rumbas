#!/bin/sh

rm -rf defaults custom_part_types exams questions question_templates exam_templates resources _output .rumbas themes

rumbas init

# Set resources
cp -r numbas_resources resources/question-resources

cp question_preview.yaml exam_templates/
