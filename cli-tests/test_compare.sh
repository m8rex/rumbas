#!/bin/sh
set -e

SUBPATH=".rumbas/en/exams"
./compare.sh exam-110396-getting-started.exam "$SUBPATH/Getting Started.exam" getting_started
./compare.sh exam-115828-diagnosys.exam "$SUBPATH/DIAGNOSYS.exam" diagnosys
./compare.sh exam-14065-geogebra-extension-demo.exam "$SUBPATH/GeoGebra extension demo.exam" geogebra
./compare.sh exam-119218-jesse-s-copy-of-simplex-method.exam "$SUBPATH/Simplex method.exam" simplex
./compare.sh exam-11981-numbas-website-demo.exam "$SUBPATH/Numbas website demo.exam" numbas_website_demo
