# Palautebot :link::shipit:

Telegram bot, joka lähettää viestinwebbitiimin chattiin kun Prodekon Githubiin pusketaan tavaraa.

## Käyttäminen

Komennot:

- /start
- /help
- /commits [repon nimi]
  - esim. /commits prodeko-org-djangocms

## Kehittäminen

1. Asenna java, clojure, leiningen ja ngrok.

```
$ brew cask install java
$ brew install clojure
$ brew install leiningen
$ brew install ngrok
```

2. Kopioi .envrc.example ja nimeä se .envrc. Konfiguroi tarvittavat muuttujat ja aja `source .envrc`

3. Käynnistä ngrok ajamalla `ngrok http 3000` ja kopioi ngrokin https osoite .envrc tiedostoon

4. Käynnistä kehitysserveri `lein ring server-headless` ja repl `lein repl`
