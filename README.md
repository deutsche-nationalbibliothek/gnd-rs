# gnd-rs

[![ci](https://github.com/deutsche-nationalbibliothek/gnd-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/deutsche-nationalbibliothek/gnd-rs/actions/workflows/ci.yml)

Dieses Repository enthält Tools zur Analyse und Weiterverarbeitung der [Gemeinsamen Normdatei](https://gnd.network). Es handelt sich um Entwicklungen zur Unterstützung der Forschungsarbeiten im Projekt [Automatisches Erschließungssystem](https://www.dnb.de/DE/Professionell/ProjekteKooperationen/Projekte/KI/ki_node.html) der [Deutschen Nationalbibliothek](https://www.dnb.de).

Die Tools sind nicht für den produktiven Einsatz empfohlen. Es wird kein Support geleistet.


## Commands

### skosify

```bash
$ gnd --config contrib/AEN.toml tests/data/118515551.dat.gz
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX skos: <http://www.w3.org/2004/02/skos/core#>
PREFIX gnd: <http://d-nb.info/gnd/>

gnd:118515551 a skos:Concept;
  skos:altLabel "브로흐, 헤르만"@de,
    "Broxi, Herman"@de,
    "Broch, German"@de,
    "ברוך, הרמן"@de,
    "ヘルマン・ブロッホ"@de,
    "Broch, Herman"@de,
    "Broh, Herman"@de,
    "Mproch, Cherman"@de;
  skos:hiddenLabel "הרמן ברוך"@de,
    "German Broch"@de,
    "Herman Broxi"@de,
    "Herman Broh"@de,
    "헤르만 브로흐"@de,
    "Herman Broch"@de,
    "Hermann Broch"@de,
    "Cherman Mproch"@de,
    "Broch, Hermann"@de;
  skos:prefLabel "Broch, Hermann (1886-1951)"@de.
```

### tabulate

```bash
$ gnd --config contrib/AEN.toml tabulate synonyms tests/data/118515551.dat.gz
uri,kind,synonym
http://d-nb.info/gnd/118515551,alternative,"Mproch, Cherman"
http://d-nb.info/gnd/118515551,alternative,"브로흐, 헤르만"
http://d-nb.info/gnd/118515551,hidden,"Broch, Hermann"
http://d-nb.info/gnd/118515551,hidden,German Broch
http://d-nb.info/gnd/118515551,alternative,"Broxi, Herman"
http://d-nb.info/gnd/118515551,alternative,"Broch, Herman"
http://d-nb.info/gnd/118515551,preferred,"Broch, Hermann (1886-1951)"
http://d-nb.info/gnd/118515551,hidden,Herman Broxi
http://d-nb.info/gnd/118515551,alternative,"Broch, German"
http://d-nb.info/gnd/118515551,hidden,헤르만 브로흐
http://d-nb.info/gnd/118515551,hidden,Hermann Broch
http://d-nb.info/gnd/118515551,alternative,"ברוך, הרמן"
http://d-nb.info/gnd/118515551,alternative,ヘルマン・ブロッホ
http://d-nb.info/gnd/118515551,alternative,"Broh, Herman"
http://d-nb.info/gnd/118515551,hidden,Cherman Mproch
http://d-nb.info/gnd/118515551,hidden,הרמן ברוך
http://d-nb.info/gnd/118515551,hidden,Herman Broh
http://d-nb.info/gnd/118515551,hidden,Herman Broch
```
