#!/bin/bash
wkhtmltopdf --page-size A4 --margin-top 10mm --margin-bottom 10mm --margin-right 10mm --margin-left 10mm /app/input.html /app/output.pdf
