# sun

## step 1: convert raw data (csv) to JSON

```bash
csv2json -a ~/Downloads/seattle.csv | jq > seattle.json #jq optional
```

## step 2: compile/run rust program to convert and plot SVG

`cargo run --bin json2day -- --input tucson.json   --output tucson.svg --label "Tucson, AZ" --transformed tucson-xformed.json`

## ptional: generate gnuplot plot

`./redacted.sh [tucson/seattle]`

### optional: convert SVG to png

`convert -density 300 seattle.svg seattle.png`

### optional: conv xformed JSON to CSV for import into spreadsheet

`json2csv seattle-xformed.csv >seattle-xformed.json`
