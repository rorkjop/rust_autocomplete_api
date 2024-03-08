# Matrikkel Autocomplete

## Getting/updating data files

```
cd data/
wget https://nedlasting.geonorge.no/geonorge/Basisdata/MatrikkelenAdresse/CSV/Basisdata_0000_Norge_25833_MatrikkelenAdresse_CSV.zip
unzip -j Basisdata_0000_Norge_25833_MatrikkelenAdresse_CSV.zip
xsv fmt -d ';' matrikkelenAdresse.csv | xsv select 'kommunenummer,adresseTekst,gardsnummer,bruksnummer' > adresser.filtered.csv
gzip adresser.filtered.csv
rm *.csv *.zip
```

## Buiding locally

```
cargo build
```

or

```
cargo build --release
```

Release builds takes more time to build, but loads the addresses into memory much faster than debug builds.

## Running locally

```
cargo run
```

Server is hosted on port 8080 - port can be overridden by setting a `PORT` environment variable.

## Building with docker

```
docker build .
```

## Running with docker locally

```
docker run --rm -it -p 8080:8080 (docker build -q .)
```

Hit Ctrl+C to exit.
