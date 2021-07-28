#/bin/sh
set -e

rumbas init

# Set resources
mkdir resources/question-resources
cp 640px-Numbat.jpg resources/question-resources
cp material-dw4n67kf_vCI9659.ggb resources/question-resources
cp Numbat_Face_U5OjlWd.jpg resources/question-resources
cp random-numbers_1yehVZF.svg resources/question-resources

./run.sh
